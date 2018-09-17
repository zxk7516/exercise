#include "difference_of_squares.h"

int squares::square_of_sums(int num){
    return std::pow(num,2)*std::pow(num+1,2)/4;
}

int squares::sum_of_squares(int num){
    return num*(num+1)*(2*num+1)/6;
}

int squares::difference(int num){
    return square_of_sums(num) - sum_of_squares(num);
}