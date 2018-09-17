#include "hamming.h"

int hamming::compute(std::string a, std::string b) {
    if ( a.length() != b.length()) {
        throw std::domain_error("unequal");
    }
    int hamming_dist = 0;
    for(auto i = 0;i< a.size();i++){
        if(a[i]!=b[i]){
            hamming_dist ++;
        }
    }
    return hamming_dist;
}