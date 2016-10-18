#include <iostream>
using namespace std;

void PrintDiv(int a, int b, int n)
{
    cout << a << " / " << b << " = " ;
    if (b == 0)
    {
        // if divide by 0
        cout << (a >= 0 ? "+INF": "-INF") << endl;
        return;
    }
    if (a == 0)
    {
        cout << 0 << endl;
        return;
    }
    if (n <= 0)
    {
        // just the integer part
        cout << a / b << endl;
        return;
    }
    if (((a > 0) && (b < 0)) || ((a < 0) && (b > 0)))
    {
        // check the sign
        cout << "-";
        a = a > 0 ? a: -a;
        b = b > 0 ? b: -b;
    }
    int c = a / b;
    for (int i = 0; i < n; i ++) // iterated
    {
        cout << c;
        a -= b * c;
        if (a == 0) break; // full division no need to continue
        a *= 10;
        c = a / b;
        if (i == 0) cout << ".";
    }
    cout << endl;
}

int main()
{
    cout << "Please give me three integers: " << endl;
    int a, b, n;
    do {
        cin >> a >> b >> n;
        PrintDiv(a, b, n);
    } while (n >= 0);
    cin >> n;
}
