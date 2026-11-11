#define _CRT_RAND_S 
#include <stdlib.h>
#include <stdio.h>

#define ORDER_NUMBER 100
#define PRODUCT_PRICE 10
#define MAXIMUM_PRODUCTS 20
#define TRUE 1
#define FALSE 0
void preia_comenzi(int portii[], bool livrari[])
{
    unsigned int randomValue= 10;

    #pragma omp parallel for shared(portii, livrari) private(randomValue)
    for(int i = 0 ; i < ORDER_NUMBER; i++)
    {
        
        rand_s(&randomValue);
        portii[i] = randomValue % MAXIMUM_PRODUCTS + 1;
        rand_s(&randomValue);
        livrari[i] = randomValue % 2;
    }

    // printf("%d", randomValue);
}

void calculeaza_produse(int portii[], int subtotal[], int pret)
{
    #pragma omp parallel for shared(portii,subtotal) firstprivate(pret)
    for(int i = 0 ; i < ORDER_NUMBER ; i ++)
    {
        subtotal[i] = portii[i] * pret;
        pret++;
    }
    // printf("%d Pretul", pret);

}

void aplica_reducere(int portii[], int subtotal[])
{
    #pragma omp parallel for shared(portii,subtotal)
    for(int i = 0 ; i < ORDER_NUMBER; i++)
    {
        if( portii[i] >= 4)
        {
            subtotal[i] = (int)(subtotal[i] * 0.9f);
        }
    }
}

void adauga_livrare(bool livrari[], int subtotal[])
{
    for(int i = 0 ; i < ORDER_NUMBER; i++)
    {
        if(livrari[i] == TRUE)
        {
            subtotal[i] += 5;
        }
    }
}


void pregateste_bon(int numar_bon[])
{
    for(int i = 0; i < ORDER_NUMBER; i++)
    {
        numar_bon[i] = 1001 + i;
    }
}

void incasari_totale ( int subtotal[], int &suma)
{
    #pragma omp parallel for shared(subtotal) reduction (+:suma)
    for(int i = 0 ; i < ORDER_NUMBER; i++)
    {
        suma+= subtotal[i];
    }
}

void comenzi_livrare( bool livrare[], int &numar)
{
    #pragma omp parallel for shared(livrare, numar)
    for(int i = 0 ; i < ORDER_NUMBER; i++)
    {
        if(livrare[i] == true)
        {
            #pragma omp atomic update
            numar++;
        }
    }
}

void comanda_maxima(int numar_bon[], int subtotal[],
    int &numar_bon_maxim, int &pret_maxim)
{
    pret_maxim = -1;
    numar_bon_maxim = -1;

    #pragma omp parallel for shared (numar_bon, subtotal,numar_bon_maxim, pret_maxim)
    for(int i = 0 ; i< ORDER_NUMBER; i++)
    {
        if(subtotal[i] > pret_maxim)
        {
            #pragma omp critical(maxim)
            {
                pret_maxim = subtotal[i];
                numar_bon_maxim = numar_bon[i];
            }
        }
    }
}

void distributie_produse(int distributie_portii[], int portii[] )
{
    #pragma omp parallel for shared (distributie_portii)
    for(int i = 0 ; i < ORDER_NUMBER; i++)
    {
        #pragma omp atomic update
        distributie_portii[portii[i]]++;
    }
}
void raport_final(int suma, int numar, int maxim, int bon, int distributie[])
{
    printf("Venitul total din ziua curenta este: %d \n", suma);
    printf("Numarul comenzilor cu livrare este: %d \n ", numar);
    printf("Comanda maxima a avut bonul %d si a valorat %d \n", bon, maxim);
    for( int i = 1 ; i <= MAXIMUM_PRODUCTS; i++)
    {
        printf("%d comenzi au avut %d produse\n", distributie[i], i);
    }
    
}

int main()
{
    int portii[ORDER_NUMBER];
    bool livrare[ORDER_NUMBER];
    int subtotal[ORDER_NUMBER];
    int numar_bon[ORDER_NUMBER];
    int distributie_portii[MAXIMUM_PRODUCTS + 1] = {0};
    int suma = 0;
    int numar_comenzi_livrare = 0;
    int numar_maxim_bon = 0;
    int valoare_maxima_comanda = 0;


    preia_comenzi(portii, livrare);
    calculeaza_produse(portii, subtotal, PRODUCT_PRICE);
    aplica_reducere(portii, subtotal);
    adauga_livrare(livrare,subtotal);
    pregateste_bon(numar_bon);
    incasari_totale(subtotal, suma);
    comenzi_livrare(livrare ,numar_comenzi_livrare);
    comanda_maxima(numar_bon, subtotal, numar_maxim_bon, valoare_maxima_comanda);
    distributie_produse(distributie_portii, portii);

    raport_final(suma, numar_comenzi_livrare, valoare_maxima_comanda, numar_maxim_bon, distributie_portii);

    // for(int i = 0 ; i < ORDER_NUMBER; i++)
    // {
    //     printf(" \n Comanda %d are %d produse, costa %f si are bonul cu numarul %d", i, portii[i], subtotal[i], numar_bon[i]);

    // }


}