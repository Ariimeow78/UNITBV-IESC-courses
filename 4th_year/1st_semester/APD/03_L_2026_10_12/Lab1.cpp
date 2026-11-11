#include <cstdlib>
#include <iostream>
#include <omp.h>

#define PORTION_NUMBER 20
#define ORDER_NUMBER 4 

int order_number;

void calculeaza_produse(
    int portii[],
    int subtotal[],
    int pret_portii,
    int reducere[]
) {

    int randomNum = rand() % 5;

    for(int i = 0; i < ORDER_NUMBER; i++) {
        if(portii[i] > 4) {
            reducere[i] = 1;
        }


        if(randomNum > 4)
            reducere[i] = 1;

        subtotal[order_number] = portii[i] * 10;
    }
}

void aplica_reducere(
    int portii[],
    int subtotal[],
    int redus[]
) {

    for(int i = 0; i < PORTION_NUMBER; i++) {
        if(portii[i] >= 4)
            redus[i] = 1;
    }

    for(int i = 0; i < ORDER_NUMBER; i++) {
        if(redus[i] == 1)
            subtotal[i] -= (subtotal[i]/10);
    }

}

void adauga_livrare(
    bool livrare[],
    int subtotal[]
) {
    
    for(int i = 0; i < order_number; i++) {
        if(livrare[i] == 1)
            subtotal[i] += 5;
    }
}


void preia_comanda(
    int portii[],
    bool livrari[]
) {
    
    for(int i = 0; i < PORTION_NUMBER; i++ ) {
        int randomNum        = (rand() % 4) + 1;
        int randomNumLivrari = rand() % 2;

        portii[i]  = randomNum;
        livrari[i] = randomNumLivrari;
    }
}

int main() {

    /* livrare = true/false bazat pe numarul de comenzi */
    /* void preia_comanda(int portii[], bool livrare[]) */
    int portii[20];
    bool livrare[4];

    std::cout << "Hello world!" << std::endl;

    return 0;
}
