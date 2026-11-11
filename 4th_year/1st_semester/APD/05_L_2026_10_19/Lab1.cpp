#include <cstdio>
#include <cstdlib>
#include <omp.h>
#include <chrono>

#define PORTION_NUMBER 20
#define ORDER_NUMBER 10

// #pragma omp parallel for
// #pragma omp parallel for shared()

void preia_comanda(
    int portii[],
    bool livrari[]
) {

#pragma omp parallel for shared(portii, livrari)
    for(int i = 0; i < ORDER_NUMBER; i++ ) {
        int randomNum        = (rand() % 4) + 1;
        int randomNumLivrari = rand() % 2;
        
        portii[i]  = randomNum;
        livrari[i] = randomNumLivrari;
    }
}

void calculeaza_produse(
    int portii[],
    int subtotal[],
    int pret_portie,
    bool reducere[]
) {

#pragma omp parallel for shared(portii, subtotal, reducere) firstprivate(pret_portie)
    for(int i = 0; i < ORDER_NUMBER; i++) {
        if(portii[i] == 4) {
            reducere[i] = true;
        } else {
            reducere[i] = false;
        }

        subtotal[i] = portii[i] * pret_portie;
    }
}

void aplica_reducere(
    int subtotal[],
    bool redus[]
) {
#pragma omp parallel for shared(subtotal, redus)
    for(int i = 0; i < ORDER_NUMBER; i++) {
        if(redus[i] == 1)
            subtotal[i] -= (subtotal[i]/10);
    }
}

void adauga_livrare(
    bool livrare[],
    int subtotal[]
) {
    
#pragma omp parallel for shared(livrare, subtotal)
    for(int i = 0; i < ORDER_NUMBER; i++) {
        if(livrare[i] == 1)
            subtotal[i] += 5;
    }
}

void pregatire_bon(int numar_bon[]) {
#pragma omp parallel for shared(numar_bon)
    for(int i = 0; i < ORDER_NUMBER; i++) {
       numar_bon[i]  = 1001 + i;
    }
}

int main() {

    /* livrare = true/false bazat pe numarul de comenzi */
    /* void preia_comanda(int portii[], bool livrare[]) */

    int  portii[ORDER_NUMBER];
    bool livrare[ORDER_NUMBER];
    bool reducere[ORDER_NUMBER];
    int  subtotal[ORDER_NUMBER];
    int  numar_bon[ORDER_NUMBER];

    auto start = std::chrono::high_resolution_clock::now();
    preia_comanda(portii, livrare);
    auto stop  = std::chrono::high_resolution_clock::now();

    std::printf("Time elapsed %lld microseconds\n",
        (long long)std::chrono::duration_cast<std::chrono::microseconds>(stop-start).count());

    calculeaza_produse(portii, subtotal, 10, reducere);
    aplica_reducere(subtotal, reducere);
    adauga_livrare(livrare, subtotal);
    pregatire_bon(numar_bon);

    for(int i = 0; i < ORDER_NUMBER; i++)
        std::printf("Prețul comenzii %d este %d și are bonul %d\n", i, subtotal[i], numar_bon[i]);

    return 0;
}
