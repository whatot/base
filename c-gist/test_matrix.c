/*
 * Origin : http://www.slyar.com/blog/matrixmutiply-c.html
 * */

#include <stdio.h>
#include <stdlib.h>

#define MATRIX_DIMENSION (1000)

/* 给 int 类型定义别名 matrix_t */
typedef long int matrix_t;

/* 函数声明部分 */
extern long int random(void);
static void set_matrix_dimension(void);
matrix_t **create_matrix(int m, int n);
void reset_matrix(matrix_t **, int, int);
void random_matrix(matrix_t **, int, int);
void output_matrix(matrix_t **, int, int);
void matrix_mutiply(matrix_t **, matrix_t **, matrix_t **);
void matrix_mutiply_fast(matrix_t **, matrix_t **, matrix_t **);
void matrix_free(matrix_t **, int);

/* 定义三个矩阵的行列大小 */
int row_a, col_a;
int row_b, col_b;
int row_c, col_c;

static FILE *fp;

int main(void)
{
    /* int i; */
    matrix_t **a, **b, **c;

    set_matrix_dimension();

    /* 创建并读入矩阵a */
    a = create_matrix(row_a, col_a);
    random_matrix(a, row_a, col_a);

    /* 创建并读入矩阵b */
    b = create_matrix(row_b, col_b);
    random_matrix(b, row_b, col_b);

    /* 以写入方式打开输出文件 out.txt */
    if ((fp = fopen("out.txt", "w")) == NULL) {
        printf("Cannot open this file.\n");
        exit(0);
    }

    /* 判断两个矩阵能否相乘 */
    if (col_a == row_b) {
        row_c = row_a;
        col_c = col_b;
    } else {
        fprintf(fp, "Matrix Can't Mutiply !\n");
        exit(0);
    }

    /* 创建并初始化结果矩阵c */
    c = create_matrix(row_c, col_c);
    reset_matrix(c, row_c, col_c);

    /* 进行矩阵乘法运算 */
    matrix_mutiply(a, b, c);

    /* 输出结果矩阵C */
    output_matrix(c, row_c, col_c);

    /* 关闭输出文件 */
    fclose(fp);

    /* 释放矩阵内存 */
    matrix_free(a, row_a);
    matrix_free(b, row_b);
    matrix_free(c, row_c);

    return 0;
}

static void set_matrix_dimension(void)
{
    col_a = col_b = col_c = MATRIX_DIMENSION;
    row_a = row_b = row_c = MATRIX_DIMENSION;
}

/* 为矩阵动态分配内存的函数 */
matrix_t **create_matrix(int m, int n)
{
    int i;
    matrix_t **Matrix;
    Matrix = (matrix_t **) malloc(sizeof(matrix_t *) * m);
    for (i = 0; i < m; i++) {
        Matrix[i] = (matrix_t *) malloc(sizeof(matrix_t) * n);
    }
    return Matrix;
}

/* 初始化矩阵函数 */
void reset_matrix(matrix_t ** Matrix, int m, int n)
{
    int i, j;
    for (i = 0; i < m; i++) {
        for (j = 0; j < n; j++) {
            Matrix[i][j] = 0;
        }
    }
}

/* 使用random()填充矩阵 */
void random_matrix(matrix_t ** Matrix, int m, int n)
{
    int i, j;
    for (i = 0; i < m; ++i) {
        for (j = 0; j < n; ++j) {
            Matrix[i][j] = random();
        }
    }
}


/* 输出数据函数 */
void output_matrix(matrix_t ** Matrix, int m, int n)
{
    int i, j;
    for (i = 0; i < m; i++) {
        for (j = 0; j < n; j++) {
            fprintf(fp, "%ld ", Matrix[i][j]);
        }
        fprintf(fp, "\n");
    }
}

/* 矩阵乘法运算函数 */
void matrix_mutiply(matrix_t ** a, matrix_t ** b, matrix_t ** c)
{
    int i, j, k;
    for (i = 0; i < row_c; i++) {
        for (j = 0; j < col_c; j++) {
            for (k = 0; k < col_a; k++) {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
}

/* 矩阵乘法运算函数，可能更快 */
void matrix_mutiply_fast(matrix_t ** a, matrix_t ** b, matrix_t ** c)
{
    int i, j, k, temp;
    for (i = 0; i < row_c; i++) {
        for (j = 0; j < col_c; j++) {
            temp = a[i][j];
            for (k = 0; k < col_a; k++) {
                c[i][k] += temp * b[j][k];
            }
        }
    }
}

/* 释放矩阵内存函数 */
void matrix_free(matrix_t ** Matrix, int m)
{
    int i;
    for (i = 0; i < m; i++) {
        free(Matrix[i]);
    }
    free(Matrix);
}
