/*
 *      Name:   Towers of Hanoi
 *
 *      Desc:
 *              This is a linux copy of towers of hanoi. It's desighed for
 *              my C-language and DataStruct task.
 *
 *      Author: 陈东坡 , Jesson Chan
 *      E-mail：chinachendongpo@gmail.com
 *      Date:   23/Jul/10
 *      运行环境，Linux,Code::Blocks,Xterm.
 *      http://www.cnblogs.com/JessonChan/archive/2010/07/29/1788280.html
 *
 */


#include<stdio.h>
#include<malloc.h>
#include<string.h>
#include<stdlib.h>
#include<time.h>
#include <unistd.h>
#include<ctype.h>

#define PER_TIME 100000
#define TIME_NUM 45
//#define SLEEPTIME usleep(TIME_NUM*PER_TIME)
#define SLEEPTIME getchar()
#define UNSLEEP usleep(1*PER_TIME);//输出等待的时间
#define SWITCH 1//控制是否以图形输出

int STEP =8;
int plat = 1;//为了在windows下快速移植，当plat=0时，将自动修改一部分函数的功能
int g_level=-1;//全局
char g_tower = 'A';//两个全局变可以控制输出
int counter=0;//计算运算步骤
char shine[]="\033[44;32m\033[1m";
char flash[]="\033[46;31m\033[5m";
char reshine[]="\033[43;31m\033[1m";
char unshine[] ="\033[0m";//颜色控制
//char shine[]="";
//char flash[]="";
//char reshine[] = "";
//char unshine[]="";

//自定义结构体
typedef struct TowerType{//用来控制主程序
    int level;//表示主塔A上的数目
    char curTower;//当前塔
    char desTower;//目标塔
    char tempTower;//过渡塔
    }*Tower; //栈记录结构体
typedef struct StackType{//由于层数一定且数值较小，可采用顺序栈，方便操作
    Tower data ;
    int current ;//当前指针位置,栈顶
    int size;//栈容量，这个是由汉诺塔的层数决定
    }*Stack ;
typedef struct towerStackType{//用来记录每个塔上的个数，其中，current当前层数
    int *towers;
    int current ;
    } *towerStack;

void Logo();
char Menu();//用来显示菜单
void InitTower(Tower , int , char,char,char);//初始化主塔
void InitStack(Stack , int );//初始化主栈，用来存放解题步骤
void Push(Stack , int ,char ,char ,char );//入栈
void Pop(Stack stack);//出栈
Tower GetTop(Stack);//返回栈顶
void DestroyStack(Stack);//销毁栈
void InitTowerStack(towerStack,int);//初始化副栈，用来存放每个塔上盘子的信息
void Printf(int ,int ,char,towerStack,int);//显示某塔上某一层的数量（图形界面）,在这里才有颜色界面的出现
void Display(towerStack , char ,char);//图形显示，在这里才有颜色界面的出现
void move(char ,char ,towerStack);//图形移动函数
void Move(char ,char);//文本显示移动过程
void Hanoi(int ,char ,char,char);//递归解法
void hanoi(int);//非递归
void cmp(int );//比较两种算法
//int  FileCmp()
int main()
{
    int num =9;//总数
    char choose = 'Q';
    Logo();
    getchar();
    printf("\033[2J\033[0;0H");
    hanoi(num);
    getchar();
    while(choose!='E')
    {
        choose='E';
        printf("\033[2J\033[0;0H");
    printf("\033[5;30HR.重新演示\033[6;30HA.任意层递归演示\033[7;30HB.任意层非递归演示\033[8;30HC.两种算法运行时间比较\033[9;30HE.结束程序\n");
    scanf(" %c",&choose);
    //scanf("% c",&choose);
    choose=toupper(choose);
    counter=0;
    switch(choose)
    {
        case 'R':hanoi(num);getchar();break;
        case 'A':printf("\033[2J\033[0;0H递归演示，请输入塔的层数\n");scanf(" %d",&num);printf("\n\t\t\t\033[0,30H%d层汉诺塔递归演示\n",num);STEP= 1;Hanoi(num,'A','B','C');getchar();getchar();break;
        case 'B':printf("\033[2J\033[0;0H非递归演示，请输入塔的层数\n");scanf(" %d",&num);printf("\n\t\t\t\033[0,30H%d层汉诺塔非递归演示\n",num);STEP= 1;hanoi(num);getchar();getchar();break;
        case 'C':printf("\033[2J\033[0;0H对两种算法比较，请输入最大的层数\n");scanf(" %d",&num);cmp(num);getchar();getchar();break;
        default:break ;
    }
    }
    return 1;
}
void InitTower(Tower tower,int num,char cur, char des,char temp)//初始化栈
{
    tower->level = num ;
    tower->curTower = cur;
    tower->desTower = des ;
    tower->tempTower = temp ;
}

void InitStack(Stack stack ,int num)//num 是盘子总数，在本程序中应该是9
{
    stack->data = (Tower)malloc(sizeof(struct TowerType)*num);
    stack->current = 0 ;
    stack->size = num;
}
void Push(Stack stack ,int num ,char A,char B ,char C)//压栈
{
    if(stack->size>stack->current)//防止出界
    {
        InitTower(&stack->data[stack->current],num,A,B,C);
        stack->current++;
    }
}
void Pop(Stack stack)//弹栈
{
    if(stack->current)
    {
        stack->current--;//顺序栈，不用每一次弹栈都去释放空间
    }
}
Tower GetTop(Stack stack)//取栈顶
{
    if(stack->current)
        return &stack->data[stack->current-1];//返回地址
    else
        return (Tower)NULL;
}
void DestroyStack(Stack stack)
{
    free(stack->data);
    stack->data = NULL;
}
void InitTowerStack(towerStack tower,int num)
{
    int i =0;
    tower->towers = (int *)malloc(sizeof(int)*num);
    tower->current  =0 ;
    for(;i<num;i++)
        tower->towers[i]=0;//把数组内的数值初始化为0
}
void Printf(int num,int flag,char cur,towerStack tStack,int i)
{
    int tmp =12-num;
    char *space =(char*)malloc(sizeof(char)*25);
    memset(space,' ',25);
    memset(space,'*',2*num+1+tmp);
    memset(space,' ',tmp);
    space[12]='|';
    space[24]='\0';
    if(cur==g_tower&&tStack[cur-'A'].towers[i-1]&&!tStack[cur-'A'].towers[i]&&counter<STEP)
    {
        num = g_level;
        tmp =12-num;
        memset(space,' ',25);
        memset(space,'*',2*num+1+tmp);
        memset(space,' ',tmp);
        space[12]='|';
        space[24]='\0';
        char * ptrA = &space[tmp];
        char* ptrB = &space[2*num+1+tmp+1];
        space[tmp-1]='\0';
        space[2*num+1+tmp]='\0';
        printf("%s %s%s%s %s ",space,flash,ptrA,unshine,ptrB);
        return;
    }
    if(flag==1&&counter<STEP)
    {
        char * ptrA = &space[tmp];
        char* ptrB = &space[2*num+1+tmp+1];
        space[tmp-1]='\0';
        space[2*num+1+tmp]='\0';
        printf("%s %s%s%s %s ",space,reshine,ptrA,unshine,ptrB);
    }
    else
    printf("%s ",space);
}
void Display(towerStack tStack,char cur ,char des)
{
    int i=8;
    char * mark=(char*)malloc(sizeof(char)*78);;
    char *space = (char*)malloc(sizeof(char)*78);//显示主框
    char *line = (char*)malloc(sizeof(char)*78);//分隔线，用来分隔表现图
    char *control = (char*)malloc(sizeof(char)*78);//用来表示方向和说明图
    char *color = (char*)malloc(sizeof(char)*78);//color control
    char * ptr =NULL;
    char * ptrB = NULL;
    int delta = 7 ;
    int flag =tStack[des-'A'].towers[tStack[des-'A'].current-1];
    char struction[]="A---------B";


    memset(mark,'=',78);
    mark[77] = '\0';

    memset(color,' ',78);
    color[77]='\0';
    color[(cur-'A')*25+13]='|';
    color[(des-'A')*25+13]='|';
    memset(control,' ',78);
    control[77]='\0';
    control[(cur-'A')*25+13]='|';
    control[(des-'A')*25+13]='|';
    control[0]=control[76]='+';

    memset(line,' ',78);
    space[77]='\0';
    if(des>cur)
    {
        delta +=(des-cur-1)*13;
        struction[0]=cur;
        struction[10]=des;
        struction[9]='>';
        memset(line,'_',(des-'A')*25+14);
        memset(line,' ',(cur-'A')*25+13);
        color[(cur-'A')*25+13+delta-1]='\0';
        ptr = &color[(cur-'A')*25+13+delta];
        color[(cur-'A')*25+13+delta-2]=48+flag;
        color[(cur-'A')*25+13+delta-3]='=';
        color[(cur-'A')*25+13+delta-4]='L';
        strcpy(ptr,struction);
        ptrB=&ptr[strlen(struction)+1];
    }
    else
        if(des<cur)
        {
            delta +=(cur-des-1)*13;
            struction[10]=cur;
            struction[0]=des;
            struction[1]='<';
            memset(line,'_',(cur-'A')*25+14);
            memset(line,' ',(des-'A')*25+13);
            color[(des-'A')*25+13+delta-1]='\0';
            ptr = &color[(des-'A')*25+13+delta];
            color[(des-'A')*25+13+delta-2]=48+flag;
            color[(des-'A')*25+13+delta-3]='=';
            color[(des-'A')*25+13+delta-4]='L';
            strcpy(ptr,struction);
            ptrB=&ptr[strlen(struction)+1];
        }
        else
        {
            goto start ;
        }
    line[0] = line[76] = '+';

    memset(space,'-',78);
    space[77]='\0';
    space[0] = space[76] = '+';
    printf("%s\n",mark);
    printf("+\t    塔A\t\t\t     塔B\t\t      塔C\t    +\n");
    printf("%s\n",line);
    printf("%s\n",control);
    printf("%s%s %s %s%s\n",color,shine,ptr,unshine,ptrB);
    printf("%s\n",space);
start:    ;
    while(i>-1)
    {        printf("+");
            Printf(tStack[0].towers[i],des=='A'&&i==tStack[0].current-1,'A',tStack,i);
            Printf(tStack[1].towers[i],des=='B'&&i==tStack[1].current-1,'B',tStack,i);
            Printf(tStack[2].towers[i],des=='C'&&i==tStack[2].current-1,'C',tStack,i);
            printf("+");
            printf("\n");
        i--;
    }
    printf("+\t    塔A\t\t\t     塔B\t\t      塔C\t    +\n");
    printf("%s\n",mark);
    if(counter<STEP)
   {
    printf("\n\t\t注：浅蓝闪烁表示\033[;35m搬离\033[0m该层，黄底红星表示\033[;35m搬到\033[0m该层。\n");
    SLEEPTIME;
   }
    else
    UNSLEEP;
    if(counter<511)
    printf("\033[2J\033[0;0H\n");
    free(mark);
    free(space);
    free(control);
    free(line);
    free(color);
}
void move(char cur ,char des,towerStack tStack)
{
    tStack[cur-'A'].current--;
    tStack[des-'A'].towers[tStack[des-'A'].current]=tStack[cur-'A'].towers[tStack[cur-'A'].current];//控制入栈
    tStack[cur-'A'].towers[tStack[cur-'A'].current]=0;//出栈
    tStack[des-'A'].current++;
    g_level = tStack[des-'A'].towers[tStack[des-'A'].current-1];
    g_tower = cur ;
    Display(tStack,cur,des);
}
void Move(char A , char B)
{
     printf("%c--->>>%c\n",A,B);
     if(counter<STEP)
     SLEEPTIME;
     else
     UNSLEEP;
}
void Hanoi(int n , char A ,char B ,char C)
{
    if(n==1)
    {
     counter++;
     if(STEP)
    { printf("第%d步：\n",counter);
     Move(A,C);}
    }
    else
    {
        Hanoi(n-1,A,C,B);
        counter++;
        if(STEP)
        {
        printf("第%d步：\n",counter);
        Move(A,C);
        }
        Hanoi(n-1,B,A,C);
    }
}
void hanoi(int num)
{
    FILE* hanoi =NULL;
    Tower tower = NULL;
    char fileName[20]="hanoi";
    Stack stack = (Stack)malloc(sizeof(struct StackType));
    towerStack tStack=(towerStack)malloc(sizeof(struct towerStackType)*3);//分别表示三个塔
    InitTowerStack(&tStack[0],num);
    InitTowerStack(&tStack[1],num);
    InitTowerStack(&tStack[2],num);

       fileName[5]=num+48;
       fileName[6]='\0';
       strcat(fileName,".csv");
    hanoi = fopen(fileName,"w");
    while(tStack[0].current<num)//主塔
    {
        tStack[0].towers[tStack[0].current] = num-tStack[0].current;
        tStack[0].current++;
    }

    InitStack(stack , num);

    if(num%2)
        Push(stack,num,'A','C','B');
    else
        Push(stack,num,'A','B','C');
    if(num==9&&SWITCH&&STEP)
    {
       printf("汉诺塔初始态：\n\n");
    Display(tStack,'A','A');
    }
    while(stack->current)//栈不空
    {
        tower = GetTop(stack);
        while(tower->level!=1)
        {
            Push(stack,tower->level-1,tower->curTower,tower->tempTower,tower->desTower);
            tower = GetTop(stack);
        }
        Pop(stack);
        counter++;
        if(STEP)
       {
       printf("非递归,第%d步：\n",counter);
          fprintf(hanoi,"%d,%c,%c,%d\n",counter,tower->curTower,tower->desTower,g_level);
          if(num==9&&SWITCH&&STEP)
            move(tower->curTower,tower->desTower,tStack);
        else
            Move(tower->curTower,tower->desTower);
       }
        if(stack->current)//栈不空
        {
        tower = GetTop(stack);
        Pop(stack);
        counter++;
        if(STEP)
        {
        printf("非递归,第%d步：\n",counter);
        fprintf(hanoi,"%d,%c,%c,%d\n",counter,tower->curTower,tower->desTower,g_level);
          if(num==9&&SWITCH)
            move(tower->curTower,tower->desTower,tStack);
        else
            Move(tower->curTower,tower->desTower);
        }
        Push(stack,tower->level-1,tower->tempTower,tower->desTower,tower->curTower);
        }
    }
    if(STEP)
    printf("%s演示完成，文件写入%s%s\n",shine,fileName,unshine);
    DestroyStack(stack);
    free(stack);
    free(tStack);
    fclose(hanoi);
}
void cmp(int num)
{
    FILE* file = NULL;
    STEP=0;
    clock_t start,finish ;
    clock_t startA,finishA ;
    double durationA ;
    double durationB ;
    int i = 1 ;
    char fileName[20]="cmp";
       fileName[3]=num+48;
       fileName[4]='\0';
       strcat(fileName,".csv");
    num+=1;
    file = fopen(fileName,"w");
    for(;i<num;i++)
    {
        start=clock();
        Hanoi(i,'A','B','C');
        finish=clock();
        durationA =(double)(finish - start) / CLOCKS_PER_SEC;
        startA=clock();
        hanoi(i);
        finishA=clock();
        durationB =(double)(finishA - startA) / CLOCKS_PER_SEC;
        printf("%d层时，递归:%f , 非递归:%f,diff:%f rate:%f\n",i,durationA,durationB,durationA-durationB,(double)(finishA - startA) /(finish - start));
        fprintf(file,"%d,%d,%d\n",i,(int)(finish - start),(int)(finishA- startA));

    }
    printf("%s比较完成，写入文件%s,回车继续%s\n",shine,fileName,unshine);
    fclose(file);
}
void Logo()
{
    time_t timep;
    time(&timep);
    printf("\n\t\t\t    C语言与数据结构综合实习\n\n");
    if(plat)printf("\033[44;33m\033[1m\033[42;33m");
    printf("\t    .......................................................\n");
    printf("\t    ..00..........00...0......00...0......00....00...0.....\n");
    printf("\t    ..00..........00...0......00...0......00....00...0.....\n");
    printf("\t    ....0..0000000000...00.000000000000...00.000000000000..\n");
    printf("\t    ........00....00....00....00...0......00....00...0.....\n");
    printf("\t    ........00....00....00....00...0......00....00...0.....\n");
    printf("\t    .0...00.00....00............0........000000...000......\n");
    printf("\t    ..00.00.00...0.....000.000000000000...00....00...0.....\n");
    printf("\t    ..00.00.00...0.....000.000000000000...00....00...0.....\n");
    printf("\t    ....0.....0..0......00......0.........00...000000000...\n");
    printf("\t    ....0.....0..0......00....00000000....00.00.........0..\n");
    printf("\t    ....0.....0..0......00....00000000....00.00.........0..\n");
    printf("\t    .000.......00.......00.00.00....00....00....00000000...\n");
    printf("\t    ..00......0..0......000..000....00....00000.00....00...\n");
    printf("\t    ..00......0..0......000..000....00....00000.00....00...\n");
    printf("\t    ..00....00....00....00.00.00....00...000....00....00...\n");
    printf("\t    ..00.000........0.........00000000..........00000000...\n");
    printf("\t    .......................................................\n");
    printf("\033[0m");
    printf("作者：陈东坡\n");
    printf("题目：九层汉诺塔模拟演示\t\t\t");
    if(plat)
    printf("%s\033[?25l",ctime(&timep));
}
char Menu()
{
    char display='x';
    printf("\t请选择演示类型:\n\n");
loop :;//使用goto
    printf("\t\ta.图形显示\n");
    printf("\t\tb.文本显示\n");
    scanf(" %c",&display);
    if(display=='a'||display=='b')
        return display;
    else
    {
        printf("\033[2J\033[0;0H输入有误，请重新输入。\n");
        goto loop ;
    }
}
//int  FileCmp()
//{
//
//    FILE *fptrA=NULL;
//    FILE *fptrB=NULL;
//    char stringA[12] ;
//    char stringB[12] ;
//    int flag = 0 ;
//    int counter = 0 ;
//    fptrA = fopen("A.txt","r");
//    fptrB = fopen("B.txt","r");
//    do
//    {
//        fscanf(fptrA,"%s",stringA);
//        fscanf(fptrB,"%s",stringB);
//        counter++;
//        flag = strcmp(stringA,stringB);
//        //printf("%d\n",flag);
//        if(flag)
//        {
//            printf("Sorry, Wrong@%d!\n",counter);
//                fclose(fptrB);
//                 fclose(fptrA);
//            return flag ;
//        }
//    }while(!feof(fptrA));
//    printf("well done@%d\n",counter);
//    fclose(fptrB);
//    fclose(fptrA);
//    return flag ;
//}
