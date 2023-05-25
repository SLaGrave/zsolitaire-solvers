#include <iostream>
#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

#include "./cribbage-rs/cribbage-rs.h"

using namespace std;

///////////////////////////////////////////
// Config consts
///////////////////////////////////////////

///////////////////////////////////////////
// Main function
///////////////////////////////////////////
int main(void) {
  char colA[14], colB[14], colC[14], colD[14];

  cout << "Please enter the values of column A:" << endl;
  cin >> colA;
  cout << "\tColumn A contains: " << colA << endl;

  cout << "Please enter the values of column B:" << endl;
  cin >> colB;
  cout << "\tColumn B contains: " << colB << endl;

  cout << "Please enter the values of column C:" << endl;
  cin >> colC;
  cout << "\tColumn C contains: " << colC << endl;

  cout << "Please enter the values of column D:" << endl;
  cin >> colD;
  cout << "\tColumn D contains: " << colD << endl;

  bool success = run_simulation(colA, colB, colC, colD);
  cout << success << endl;
}