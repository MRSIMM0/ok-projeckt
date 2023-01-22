#include <stdio.h>
#include <pthread.h>
#include <unistd.h>
#include <sys/wait.h>
#include <string.h>
#include <stdlib.h>
#include <fcntl.h>

#define NUM_THREADS 2

struct data
{
    char filename[128];
    int temp;
    float coolRate;
    int iters;
};

struct ans
{
    float path;
    int temp;
    float coolRate;
    int iters;
};

char *intToString(int x, char *str)
{
    sprintf(str, "%d", x);
    return str;
}

char *floatToString(float x, char *str)
{
    sprintf(str, "%f", x);
    return str;
}

void *readOutput(void *info)
{
    struct data *my_data;

    my_data = (struct data *)info;

    char temp[20];
    char coolRate[20];
    char iters[20];

    intToString(my_data->temp, temp);
    floatToString(my_data->coolRate, coolRate);
    intToString(my_data->iters, iters);

    int fd[2];

    char buf[1028];

    pipe(fd);

    if (fork() == 0)
    {

        close(fd[0]);
        dup2(fd[1], 1);
        close(fd[1]);

        execl("/bin/sh", "-c", "./train.sh", my_data->filename, temp, coolRate, iters, NULL);
    }
    else
    {
        close(fd[1]);
        wait(NULL);
        read(fd[0], buf, 1028);
        float *f = malloc(sizeof(float));
        *f = (float)atof(buf);
        return (void *)f;
    }
}

float avg(float results[NUM_THREADS])
{
    int i = 0;
    float sum = 0;

    for (i = 0; i < NUM_THREADS; i++)
    {
        sum += results[i];
    }
    return sum / NUM_THREADS;
}

float min(float results[NUM_THREADS])
{

    float min = 99999999999;

    for (int i = 0; i < NUM_THREADS; i++)
    {
        if (results[i] < min)
        {
            min = results[i];
        }
    }
}

float evaluate(struct data d)
{
    pthread_t threads[NUM_THREADS];

    int rc, i;

    float *out;

    float res[NUM_THREADS];

    for (i = 0; i < NUM_THREADS; i++)
    {
        pthread_create(&threads[i], NULL, readOutput, (void *)&d);
    }

    for (i = 0; i < NUM_THREADS; i++)
    {
        pthread_join(threads[i], (void **)&out);
        res[i] = *out;
        free(out);
    }

    return (avg(res));
}

int main(int argc, char **argv)
{
    struct ans bestAns;

    struct data d;

    printf("%d", argc);

    if (argc < 2)
    {
        strcpy(d.filename, "./data.txt");
    }
    else
    {
        strcpy(d.filename, argv[1]);

    }

    bestAns.path = 9999999999;
    for (int i = 10000; i < 1000000; i += 200)
    {
        d.iters = i;
        for (int t = 100; t < 5000000; t += 200)
        {

            d.temp = t;
            for (float c = 0.8; c < 0.9; c += 0.1)
            {
                    d.coolRate = c;
                    float path = evaluate(d);

                    if (path < bestAns.path)
                    {
                        struct ans newAns;
                        newAns.path = path;
                        newAns.temp = t;
                        newAns.coolRate = c;
                        newAns.iters = i;
                        bestAns = newAns;
                        printf("Path Len: %f\nTemperature: %d\nCool Rate: %f\nIterations: %d \n\n", path, t, c, i);
                    }

                    closefrom(3);
            }
        }
    }

    return 0;
}