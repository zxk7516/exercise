#ifndef GIGASECOND_H
#define GIGASECOND_H

#include "boost/date_time/posix_time/posix_time.hpp"

// See <http://www.boost.org/doc/libs/1_59_0/doc/html/date_time/posix_time.html>
// for documentation on boost::posix_time

using namespace boost::posix_time;

namespace gigasecond {
    ptime advance(ptime start);
}

#endif