#include <math.h>
#include <time.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>

#define max(a, b) ((a)>(b)?(a):(b))
#define min(a, b) ((a)>(b)?(b):(a))

#define MAX_X 20000
#define MAX_Y 20000

/* $ gcc -g -Wall -o detect.bin detect_intersect.c
 * 来源： http://blog.csdn.net/rickliuxiao/article/details/6259322
 * */


///----------alg 1------------
typedef struct {
	double x, y;
} Point;

bool between(double a, double X0, double X1)
{
	double temp1 = a - X0;
	double temp2 = a - X1;
	if ((temp1 < 1e-8 && temp2 > -1e-8) || (temp2 < 1e-6 && temp1 > -1e-8)) {
		return true;
	} else {
		return false;
	}
}

// 判断两条直线段是否有交点，有则计算交点的坐标
// p1,p2是直线一的端点坐标
// p3,p4是直线二的端点坐标
bool detect_intersect_1(Point p1, Point p2, Point p3, Point p4)
{
	double line_x, line_y;	//交点
	if ((fabs(p1.x - p2.x) < 1e-6) && (fabs(p3.x - p4.x) < 1e-6)) {
		return false;
	} else if ((fabs(p1.x - p2.x) < 1e-6))	//如果直线段p1p2垂直与y轴
	{
		if (between(p1.x, p3.x, p4.x)) {
			double k = (p4.y - p3.y) / (p4.x - p3.x);
			line_x = p1.x;
			line_y = k * (line_x - p3.x) + p3.y;

			if (between(line_y, p1.y, p2.y)) {
				return true;
			} else {
				return false;
			}
		} else {
			return false;
		}
	} else if ((fabs(p3.x - p4.x) < 1e-6))	//如果直线段p3p4垂直与y轴
	{
		if (between(p3.x, p1.x, p2.x)) {
			double k = (p2.y - p1.y) / (p2.x - p1.x);
			line_x = p3.x;
			line_y = k * (line_x - p2.x) + p2.y;

			if (between(line_y, p3.y, p4.y)) {
				return true;
			} else {
				return false;
			}
		} else {
			return false;
		}
	} else {
		double k1 = (p2.y - p1.y) / (p2.x - p1.x);
		double k2 = (p4.y - p3.y) / (p4.x - p3.x);

		if (fabs(k1 - k2) < 1e-6) {
			return false;
		} else {
			line_x = ((p3.y - p1.y) - (k2 * p3.x - k1 * p1.x))
				/ (k1 - k2);
			line_y = k1 * (line_x - p1.x) + p1.y;
		}

		if (between(line_x, p1.x, p2.x) && between(line_x, p3.x, p4.x)) {
			return true;
		} else {
			return false;
		}
	}
}

///------------alg 1------------



///------------alg 2------------
//叉积
double mult(Point a, Point b, Point c)
{
	return (a.x - c.x) * (b.y - c.y) - (b.x - c.x) * (a.y - c.y);
}

//aa, bb为一条线段两端点 cc, dd为另一条线段的两端点
//相交返回true, 不相交返回false
bool detect_intersect_2(Point aa, Point bb, Point cc, Point dd)
{
	if (max(aa.x, bb.x) < min(cc.x, dd.x)) {
		return false;
	}
	if (max(aa.y, bb.y) < min(cc.y, dd.y)) {
		return false;
	}
	if (max(cc.x, dd.x) < min(aa.x, bb.x)) {
		return false;
	}
	if (max(cc.y, dd.y) < min(aa.y, bb.y)) {
		return false;
	}
	if (mult(cc, bb, aa) * mult(bb, dd, aa) < 0) {
		return false;
	}
	if (mult(aa, dd, cc) * mult(dd, bb, cc) < 0) {
		return false;
	}
	return true;
}

///------------alg 2------------



//算法3：http://dec3.jlu.edu.cn/webcourse/t000096/graphics/chapter5/01_1.
///------------alg 3------------
double determinant(double v1, double v2, double v3, double v4)	// 行列式
{
	return (v1 * v3 - v2 * v4);
}

bool detect_intersect_3(Point aa, Point bb, Point cc, Point dd)
{
	double delta =
	    determinant(bb.x - aa.x, cc.x - dd.x, bb.y - aa.y, cc.y - dd.y);
	if (delta <= (1e-6) && delta >= -(1e-6))
	{
		// delta=0，表示两线段重合或平行
		return false;
	}
	double namenda =
	    determinant(cc.x - aa.x, cc.x - dd.x, cc.y - aa.y,
			cc.y - dd.y) / delta;
	if (namenda > 1 || namenda < 0) {
		return false;
	}
	double miu =
	    determinant(bb.x - aa.x, cc.x - aa.x, bb.y - aa.y,
			cc.y - aa.y) / delta;
	if (miu > 1 || miu < 0) {
		return false;
	}
	return true;
}

///------------alg 3------------

int main()
{
	Point p1, p2, p3, p4;
	p1.x = 1;
	p1.y = 4;
	p2.x = 3;
	p2.y = 0;
	p3.x = 0;
	p3.y = 1;
	p4.x = 4;
	p4.y = 3;

	int i = 0, j = 0;
	bool flag = false;

	// alg 1
	time_t seconds1 = time(NULL);
	for (; i != MAX_X; ++i) {
		for (j = 0; j != MAX_Y; ++j) {
			flag = detect_intersect_1(p1, p2, p3, p4);
		}
	}
	time_t seconds2 = time(NULL);
	double time_usage_1 = seconds2 - seconds1;

	printf("Time used in alg 1: %f seconds.\n", time_usage_1);

	// alg 2
	time_t seconds3 = time(NULL);
	i = 0;
	j = 0;
	for (; i != MAX_X; ++i) {
		for (j = 0; j != MAX_Y; ++j) {
			flag = detect_intersect_2(p1, p2, p3, p4);
		}
	}
	time_t seconds4 = time(NULL);
	double time_usage_2 = seconds4 - seconds3;

	printf("Time used in alg 2: %f seconds.\n", time_usage_2);

	// alg 3
	time_t seconds5 = time(NULL);
	i = 0;
	j = 0;
	for (; i != MAX_X; ++i) {
		for (j = 0; j != MAX_Y; ++j) {
			flag = detect_intersect_3(p1, p2, p3, p4);
		}
	}
	time_t seconds6 = time(NULL);
	double time_usage_3 = seconds6 - seconds5;

	printf("Time used in alg 3: %f seconds.\n", time_usage_3);
	printf("flag: %d\n", flag);
	return 0;
}
