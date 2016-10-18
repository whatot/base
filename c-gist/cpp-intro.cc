#include <iostream>
#include <cstdlib>
#include <ctime>
#include <cmath>
#include <string>

using namespace std;

class Box
{
public:
    double length;
    double breadth;
    double height;
    Box(double len, double b, double h) {
        length = len;
        breadth = b;
        height = h;
        cout << "box constructor" << endl;
    }
    ~Box() {
        cout << "box destructor" << endl;
    }
};

class BigBox: public Box
{
public:
    BigBox(double len, double b, double h):Box(len, b, h) {}
    int getVolume() {
        return (length * breadth * height);
    }
};

typedef struct
{
    char title[50];
    char author[50];
    char subject[100];
    int book_id;
} Book;

void check_array(void) {
    double balance[] = {1000.0, 2.1, 3.1, 11.1, 55.5};
    for (double i : balance) {
        cout << "Sqrt of balance: " << std::sqrt(i) << endl;
    }
}

void check_class(void) {
    Box box1(10.1, 12.1, 6.6);
    cout << "The height of Box: " << box1.height << endl;
    BigBox box2(2.1, 3.2, 4.3);
    cout << "The volume of BigBox: " << box2.getVolume() << endl;
}

void check_enum(void) {
    enum color { red, green, blue };
    color c = blue;
    cout << "color of blue : " << c << endl;
}

void check_rand(void) {
    srand(std::time(0));
    for (int i = 0; i < 10; ++i) {
        int j = std::rand();
        cout << "Random number: " << i << " " << j << endl;
    }
}

void check_string(void) {
    string str1 = "Hello";
    string str2 = "World";
    cout << "str1 + str2: " << str1 + str2 << endl;
    cout << "length(str1 + str2): " << str1.size() + str2.size() << endl;
}

void check_time(void) {
    time_t now = std::time(0);
    char* dt = std::ctime(&now);
    cout << "The local date and time is:\t" << dt;

    tm *gmtm = std::gmtime(&now);
    char timebuf[100];
    if (std::strftime(timebuf, sizeof(timebuf), "%c \n", gmtm)) {
        cout << "The UTC data and time is:\t" << timebuf;
    }

    tm *ltm = std::localtime(&now);
    cout << "Year: \t" << 1900 + ltm->tm_year << endl;
    cout << "Month: \t" << 1 + ltm->tm_mon << endl;
    cout << "Day: \t" << ltm->tm_mday << endl;
    cout << "Time: \t" << ltm->tm_hour << ":";
    cout << 1 + ltm->tm_min << ":";
    cout << 1 + ltm->tm_sec << endl;
}

void check_type(void) {
    cout << "Size of char: " << sizeof(char) << endl;
    cout << "Size of int: " << sizeof(int) << endl;
    cout << "Size of short int: " << sizeof(short int) << endl;
    cout << "Size of long int: " << sizeof(long int) << endl;
    cout << "Size of float: " << sizeof(float) << endl;
    cout << "Size of double: " << sizeof(double) << endl;
    cout << "Size of wchar_t: " << sizeof(wchar_t) << endl;
}

int main() {
    cout << "Hello World" << endl;
    check_array();
    check_class();
    check_enum();
    check_rand();
    check_string();
    check_time();
    check_type();

    return 0;
}
