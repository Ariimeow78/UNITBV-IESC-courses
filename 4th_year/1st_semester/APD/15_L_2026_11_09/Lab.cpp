#include <chrono>
#include <omp.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <thread>
#include <time.h>

#define ORDER_NUMBER 200
#define PRODUCT_PRICE 10
#define MAXIMUM_PRODUCTS 20

#define INITIAL_STOCK 500

void preia_comenzi(int portii[], bool livrari[]) {
#pragma omp parallel for shared(portii, livrari)
    for (int i = 0; i < ORDER_NUMBER; i++) {
        unsigned int seed =
            (unsigned int)time(NULL) ^ (omp_get_thread_num() + 1) ^ (i * 1097);

        portii[i] = (rand_r(&seed) % MAXIMUM_PRODUCTS) + 1;
        livrari[i] = (bool)(rand_r(&seed) % 2);
    }

    // printf("%d", randomValue);
}

void acceptare_comanda(bool comenzi_acceptate[], const int portii[],
                       int &stoc) {
    // Fixed: i < ORDER_NUMBER (was i <= ORDER_NUMBER causing out of bounds)
    for (int i = 0; i < ORDER_NUMBER; i++) {
        comenzi_acceptate[i] = false;
        if (portii[i] <= stoc) {
            comenzi_acceptate[i] = true;
            stoc -= portii[i];
        }
    }
}

void pregatire_comanda(const bool comenzi_acceptate[], const int portii[],
                       bool comenzi_pregatite[], const int i) {
    comenzi_pregatite[i] = false;

    if (comenzi_acceptate[i]) {
        std::this_thread::sleep_for(std::chrono::milliseconds(10 * portii[i]));
        comenzi_pregatite[i] = true;
    }
}

void ambalare_comanda(const bool comenzi_pregatite[], const int portii[],
                      int cutii[], const int i) {
    if (comenzi_pregatite[i]) {
        std::this_thread::sleep_for(std::chrono::milliseconds(5 * portii[i]));
        cutii[i] = portii[i] / 2;
        cutii[i] += portii[i] % 2;
    }
}

void expediaza_comanda(const int cutii[], bool expediate[], bool finalizate[],
                       const int i) {
    if (cutii[i] > 0) {
        std::this_thread::sleep_for(std::chrono::milliseconds(2));
        expediate[i] = true;
        finalizate[i] = true;
    }
}

void preda_tejghea(const int cutii[], bool ridicate[], bool finalizate[],
                   const int i) {
    if (cutii[i] > 0) {
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
        ridicate[i] = true;
        finalizate[i] = true;
    }
}

void predare_comanda(const bool livrari[], const int cutii[], bool expediate[],
                     bool ridicate[], bool finalizate[], const int i) {
    if (livrari[i]) {
        expediaza_comanda(cutii, expediate, finalizate, i);
    } else {
        preda_tejghea(cutii, ridicate, finalizate, i);
    }
}

void calculeaza_produse(const int portii[], int subtotal[], const int pret,
                        const bool comenzi_acceptate[]) {
#pragma omp parallel for shared(portii, subtotal, comenzi_acceptate)           \
    firstprivate(pret)
    for (int i = 0; i < ORDER_NUMBER; i++) {
        if (comenzi_acceptate[i])
            subtotal[i] = portii[i] * pret;
        else
            subtotal[i] = 0;
    }
    // printf("%d Pretul", pret);
}

void aplica_reducere(const int portii[], int subtotal[]) {
#pragma omp parallel for shared(portii, subtotal)
    for (int i = 0; i < ORDER_NUMBER; i++) {
        if (portii[i] >= 4)
            subtotal[i] = (int)(subtotal[i] * 0.9f);
    }
}

void adauga_livrare(const bool livrari[], int subtotal[],
                    const bool comanda_acceptata[]) {
// Fixed: Removed 'portii' from shared as it is not a parameter here
#pragma omp parallel for shared(livrari, comanda_acceptata, subtotal)
    for (int i = 0; i < ORDER_NUMBER; i++) {
        if (livrari[i] && comanda_acceptata[i])
            subtotal[i] += 5;
    }
}

void pregateste_bon(int numar_bon[], const bool comenzi_acceptate[]) {
#pragma omp parallel for shared(numar_bon, comenzi_acceptate)
    for (int i = 0; i < ORDER_NUMBER; i++) {
        if (comenzi_acceptate[i])
            numar_bon[i] = 1001 + i;
        else
            numar_bon[i] = 0;
    }
}

void incasari_totale(const int subtotal[], int &suma) {
#pragma omp parallel for shared(subtotal) reduction(+ : suma)
    for (int i = 0; i < ORDER_NUMBER; i++)
        suma += subtotal[i];
}

void comenzi_livrare(const bool livrare[], int &numar) {
#pragma omp parallel for shared(livrare, numar)
    for (int i = 0; i < ORDER_NUMBER; i++) {
        if (livrare[i] == true) {
#pragma omp atomic update
            numar++;
        }
    }
}

void rezervare_stoc(const int portii[], bool comenzi_acceptate[], int &stoc) {
    for (int i = 0; i < ORDER_NUMBER; i++) {
        comenzi_acceptate[i] = false;
        if (portii[i] <= stoc) {
            comenzi_acceptate[i] = true;
            stoc -= portii[i];
        }
    }
}

// void procesare_bucatarie(int portii[], bool comenzi_acceptate[],
//                          bool comenzi_pregatite[], int cutii[], int &stoc) {
// #pragma omp parallel shared(portii, comenzi_acceptate, comenzi_pregatite, \
//                                  cutii, stoc)
//     {
// #pragma omp single
//         acceptare_comanda(comenzi_acceptate, portii, stoc);
//
// #pragma omp for schedule(auto)
//         for (int i = 0; i < ORDER_NUMBER; i++) {
//             // Fixed: passed correct arguments to pregatire_comanda
//             pregatire_comanda(comenzi_acceptate, portii, comenzi_pregatite,
//             i);
//         }
//
// // Implicit barrier exists after omp for, but an explicit one is fine
// #pragma omp barrier
//
// #pragma omp for schedule(auto)
//         for (int i = 0; i < ORDER_NUMBER; i++) {
//             // Fixed: passed correct arguments without '&' on arrays
//             ambalare_comanda(comenzi_pregatite, portii, cutii, i);
//         }
//     }
// }

void procesare_bucatarie(const int portii[], const bool livrari[],
                         bool comenzi_acceptate[], bool comenzi_pregatite[],
                         int cutii[], bool comenzi_expediate[],
                         bool comenzi_ridicate[], bool comenzi_finalizate[],
                         int &stoc) {
#pragma omp parallel shared(portii, livrari, comenzi_acceptate,                \
                                comenzi_pregatite, cutii, comenzi_expediate,   \
                                comenzi_ridicate, comenzi_finalizate, stoc)
    {
#pragma omp single
        {
            rezervare_stoc(portii, comenzi_acceptate, stoc);

#pragma omp taskgroup
            {
                for (int i = 0; i < ORDER_NUMBER; i++) {
                    if (!comenzi_acceptate[i])
                        continue;

#pragma omp task firstprivate(i)                                               \
    shared(portii, comenzi_acceptate, comenzi_pregatite)                       \
    depend(out : comenzi_pregatite[i])
                    {
                        pregatire_comanda(comenzi_acceptate, portii,
                                          comenzi_pregatite, i);
                    }

#pragma omp task firstprivate(i) shared(portii, comenzi_pregatite, cutii)      \
    depend(in : comenzi_pregatite[i]) depend(out : cutii[i])
                    {
                        ambalare_comanda(comenzi_pregatite, portii, cutii, i);
                    }

#pragma omp task firstprivate(i)                                               \
    shared(livrari, cutii, comenzi_expediate, comenzi_ridicate,                \
               comenzi_finalizate) depend(in : cutii[i])                       \
    depend(out : comenzi_finalizate[i])
                    {
                        predare_comanda(livrari, cutii, comenzi_expediate,
                                        comenzi_ridicate, comenzi_finalizate,
                                        i);
                    }
                }
            }
        }
    }
}

void comanda_maxima(const int numar_bon[], const int subtotal[],
                    int &numar_bon_maxim, int &pret_maxim) {
    pret_maxim = -1;
    numar_bon_maxim = -1;

#pragma omp parallel for shared(numar_bon, subtotal, numar_bon_maxim,          \
                                    pret_maxim)
    for (int i = 0; i < ORDER_NUMBER; i++) {
        if (subtotal[i] > pret_maxim) {
#pragma omp critical(maxim)
            {
                // Re-check inside critical section to avoid race conditions
                if (subtotal[i] > pret_maxim) {
                    pret_maxim = subtotal[i];
                    numar_bon_maxim = numar_bon[i];
                }
            }
        }
    }
}

void distributie_produse(int distributie_portii[], const int portii[]) {
#pragma omp parallel for shared(distributie_portii, portii)
    for (int i = 0; i < ORDER_NUMBER; i++) {
#pragma omp atomic update
        distributie_portii[portii[i]]++;
    }
}

// void raport_final(const int suma, const int numar, const int maxim,
//                   const int bon, const int distributie[]) {
//     printf("Venitul total din ziua curenta este: %d \n", suma);
//     printf("Numarul comenzilor cu livrare este: %d \n", numar);
//     printf("Comanda maxima a avut bonul %d si a valorat %d \n", bon, maxim);
//     for (int i = 1; i <= MAXIMUM_PRODUCTS; i++)
//         if (distributie[i] > 0)
//             printf("%d comenzi au avut %d produse\n", distributie[i], i);
// }
//
void raport_final(const bool comenzi_acceptate[],
                  const bool comenzi_finalizate[],
                  const bool comenzi_expediate[], const bool comenzi_ridicate[],
                  const int cutii[]) {
    int nr_acceptate = 0;
    int nr_finalizate = 0;
    int nr_expediate = 0;
    int nr_ridicate = 0;
    int total_cutii = 0;

#pragma omp parallel for shared(comenzi_acceptate, comenzi_finalizate,         \
                                    comenzi_expediate, comenzi_ridicate,       \
                                    cutii)                                     \
    reduction(+ : nr_acceptate, nr_finalizate, nr_expediate, nr_ridicate,      \
                  total_cutii)
    for (int i = 0; i < ORDER_NUMBER; i++) {
        nr_acceptate += comenzi_acceptate[i];
        nr_finalizate += comenzi_finalizate[i];
        nr_expediate += comenzi_expediate[i];
        nr_ridicate += comenzi_ridicate[i];
        total_cutii += cutii[i];
    }

    printf("\nComenzi acceptate :%d", nr_acceptate);
    printf("\nComenzi refuzate :%d", ORDER_NUMBER - nr_acceptate);
    printf("\nComenzi finalizate :%d", nr_finalizate);
    printf("\nComenzi expediate :%d", nr_expediate);
    printf("\nComenzi ridicate :%d", nr_ridicate);
    printf("\nTotal cutii folosite :%d\n", total_cutii);
}

int main() {
    omp_set_dynamic(0);

    int portii[ORDER_NUMBER];
    bool livrare[ORDER_NUMBER];
    int subtotal[ORDER_NUMBER];
    int numar_bon[ORDER_NUMBER];
    int distributie_portii[MAXIMUM_PRODUCTS + 1] = {0};
    int suma = 0;
    int numar_comenzi_livrare = 0;
    int numar_maxim_bon = 0;
    int valoare_maxima_comanda = 0;

    int stoc = INITIAL_STOCK;
    bool comenzi_acceptate[ORDER_NUMBER] = {false};
    bool comenzi_pregatite[ORDER_NUMBER] = {false};
    int cutii[ORDER_NUMBER] = {0};
    bool comenzi_expediate[ORDER_NUMBER] = {false};
    bool comenzi_ridicate[ORDER_NUMBER] = {false};
    bool comenzi_finalizate[ORDER_NUMBER] = {false};

    preia_comenzi(portii, livrare);

    procesare_bucatarie(portii, livrare, comenzi_acceptate, comenzi_pregatite,
                        cutii, comenzi_expediate, comenzi_ridicate,
                        comenzi_finalizate, stoc);

    calculeaza_produse(portii, subtotal, PRODUCT_PRICE, comenzi_acceptate);
    aplica_reducere(portii, subtotal);
    adauga_livrare(livrare, subtotal, comenzi_acceptate);
    pregateste_bon(numar_bon, comenzi_acceptate);

    incasari_totale(subtotal, suma);
    comenzi_livrare(livrare, numar_comenzi_livrare);
    comanda_maxima(numar_bon, subtotal, numar_maxim_bon,
                   valoare_maxima_comanda);
    distributie_produse(distributie_portii, portii);

    // raport_final(suma, numar_comenzi_livrare, valoare_maxima_comanda,
    //              numar_maxim_bon, distributie_portii);

    raport_final(comenzi_acceptate, comenzi_finalizate, comenzi_expediate,
                 comenzi_ridicate, cutii);

    for (int i = 0; i < ORDER_NUMBER; i++) {
        printf(" \n Comanda %d are %d produse, costa %d si are bonul cu "
               "numarul %d",
               i, portii[i], subtotal[i], numar_bon[i]);
    }

    printf("\n"); // <--- Add this line here
    return 0;
}
