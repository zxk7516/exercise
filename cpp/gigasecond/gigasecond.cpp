#include "gigasecond.h"

ptime gigasecond::advance(ptime start){
    time_duration td = seconds(1000000000);
    return start + td;
}
