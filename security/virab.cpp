#include <stdio.h>
#include <iostream>
using namespace std;

class a {
	public:
		void f(void) {
			cout << "base f" << endl;
		};
	
		virtual void g(void) { //虚函数
			cout << "base g" << endl;
		};
};

class b: public a {
	public:
		void f(void) {
			cout << "derived f" << endl;
		};

		void g(void) {
			cout << "derived g" << endl;
		};
};

int main(int argc, char *argv[]) {
	a *my_b = new b();
	my_b->f(); //调用a类(基类)的f()
	my_b->g(); //调用b类(派生类)的g()
	return 0;
}

