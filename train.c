#include <stdio.h>
#include <pthread.h>
#include <unistd.h>
#include <sys/wait.h>
#include <string.h>

#include <stdlib.h>

#define NUM_THREADS 7

struct data
{
    char filename[128];
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
    floatToString(my_data->coolRate,coolRate);
    intToString(my_data->iters, iters);

    int fd[2], n, pid, exit;

    char buf[1028];

    pipe(fd);

    pid = fork();
    if (pid == 0)
    {
        close(fd[0]);
        dup2(fd[1], 1);
        execl("./Simulated_Annealing/target/release/ok-sym", my_data->filename, temp, coolRate,iters, NULL);
    }
    else
    {

        close(fd[1]);
        wait(NULL);
        close(0);
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

int main()
{

    struct data d;

    strcpy(d.filename, "./data.txt");
    for (int t = 1000; t < 30000; t += 5000)
    {
        d.temp = t;
        for (float c = 0.05; c > 0.01; c -= 0.01)
        {
            d.coolRate = c;
            for (int i = 1000; i < 5000; i += 500)
            {
                d.iters = i;
                printf("%f\n", evaluate(d));
            }
        }
    }

    return 0;
}