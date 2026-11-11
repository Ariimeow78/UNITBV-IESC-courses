#include <chrono>
#include <cstdlib>
#include <omp.h>
#include <stdio.h>

#define ORDER_NUMBER 20000

void preia_comanda(int portii[], bool livrari[]) {
    int randomNum[ORDER_NUMBER];
    int randomNumLivrari[ORDER_NUMBER];
    for (int i = 0; i < ORDER_NUMBER; i++) {
        randomNum[i] = rand() % 4 + 1;
        randomNumLivrari[i] = rand() % 2;
    }

#pragma omp parallel for shared(portii, livrari)                               \
    firstprivate(randomNum, randomNumLivrari)
    for (int i = 0; i < ORDER_NUMBER; i++) {
        portii[i] = randomNum[i];
        livrari[i] = randomNumLivrari[i];
    }
}

void calculeaza_produse(int portii[], int subtotal[], int pret_portie,
                        bool reducere[]) {
#pragma omp parallel for shared(portii, subtotal, reducere)                    \
    firstprivate(pret_portie)
    for (int i = 0; i < ORDER_NUMBER; i++) {
        if (portii[i] > 4) {
            reducere[i] = 1;
        } else {
            reducere[i] = 0;
        }
        subtotal[i] = portii[i] * pret_portie;
    }
}
void aplica_reducere(int subtotal[], bool redus[]) {
#pragma omp parallel for shared(subtotal, redus)
    for (int i = 0; i < ORDER_NUMBER; i++) {
        if (redus[i] == 1) {
            subtotal[i] = subtotal[i] - (subtotal[i] / 10);
        }
    }
}
void adauga_livrare(bool livrare[], int subtotal[]) {
#pragma omp parallel for shared(subtotal, livrare)
    for (int i = 0; i < ORDER_NUMBER; i++) {
        if (livrare[i] == 1) {
            subtotal[i] += 5;
        }
    }
}

void pregatire_bon(int numar_bon[]) {
#pragma omp parallel for shared(numar_bon)
    for (int i = 0; i < ORDER_NUMBER; i++) {
        numar_bon[i] = 1001 + i;
    }
}
int main() {
    int portii[ORDER_NUMBER];

    bool livrare[ORDER_NUMBER];
    bool reducere[ORDER_NUMBER];
    int subtotal[ORDER_NUMBER];
    int numar_bon[ORDER_NUMBER];

    auto start = std::chrono::high_resolution_clock::now();
    preia_comanda(portii, livrare);
    auto stop = std::chrono::high_resolution_clock::now();
    printf("Time elapsed for preia_comanda: %lld microseconds\n",
           std::chrono::duration_cast<std::chrono::microseconds>(stop - start)
               .count());
    start = std::chrono::high_resolution_clock::now();
    calculeaza_produse(portii, subtotal, 10, reducere);
    stop = std::chrono::high_resolution_clock::now();
    printf("Time elapsed for calculeaza_produse: %lld microseconds\n",
           std::chrono::duration_cast<std::chrono::microseconds>(stop - start)
               .count());
    // stop = std::chrono::high_resolution_clock::now();
    start = std::chrono::high_resolution_clock::now();
    aplica_reducere(subtotal, reducere);
    stop = std::chrono::high_resolution_clock::now();
    printf("Time elapsed for aplica_reducere: %lld microseconds\n",
           std::chrono::duration_cast<std::chrono::microseconds>(stop - start)
               .count());
    start = std::chrono::high_resolution_clock::now();
    adauga_livrare(livrare, subtotal);
    stop = std::chrono::high_resolution_clock::now();
    printf("Time elapsed for adauga_livrare: %lld microseconds\n",
           std::chrono::duration_cast<std::chrono::microseconds>(stop - start)
               .count());
    start = std::chrono::high_resolution_clock::now();
    pregatire_bon(numar_bon);
    stop = std::chrono::high_resolution_clock::now();
    printf("Time elapsed for pregatire_bon: %lld microseconds\n",
           std::chrono::duration_cast<std::chrono::microseconds>(stop - start)
               .count());

    return 0;
}
