/*
 * =====================================================================================
 *
 *       Filename:  reciprocal.cpp
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年02月03日 11时58分09秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <cassert>
#include "reciprocal.hpp"

double reciprocal (int i) {
	// I should be non-zero
	assert (i != 0);
	return 1.0/i;
}
