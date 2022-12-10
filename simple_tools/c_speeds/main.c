#include<stdio.h>
#include <sys/time.h>
#include<stdlib.h>

#define LEN 1000000

long long current_timestamp() {
    struct timeval te; 
    gettimeofday(&te, NULL);
    long long milliseconds = te.tv_sec*1000LL + te.tv_usec/1000;
    return milliseconds;
}

typedef struct Person {
    int arr[3];
} Person;

Person** allocate_java_style_array(int len) {
    Person** res = malloc(sizeof(Person*) * len);
    int i = 0;
    while (i < len) {
        res[i] = malloc(sizeof(Person));
        i++;
    }
    return res;
}

Person** allocate_c_style_array(int len) {
    Person** res = malloc(sizeof(Person*) * len);
    Person* res_inner = malloc(sizeof(Person) * len);
    int i = 0;
    while (i < len) {
        res[i] = &res_inner[i];
        i++;
    }
    return res;
}

Person* allocate_rust_style_array(int len) {
    Person* res_inner = malloc(sizeof(Person) * len);
    return res_inner;
}

void copy_java_style_arrays(Person** source, Person** dest, int len) {
    int i = 0;
    while (i < len) {
        dest[i] = source[i];
        i++;
    }
}

void deep_copy_java_style_arrays(Person** source, Person** dest, int len) {
    int i = 0;
    while (i < len) {
        *dest[i] = *source[i];
        i++;
    }
}

void copy_rust_style_arrays(Person* source, Person* dest, int len) {
    int i = 0;
    while (i < len) {
        dest[i] = source[i];
        i++;
    }
}

void copy_c_style_arrays(Person** source, Person** dest, int len) {
    int i = 0;
    while (i < len) {
        *dest[i] = *source[i];
        i++;
    }
}

void measure_copying_of_java_style_arrays() {
    Person** source = allocate_java_style_array(LEN);
    Person** dest = allocate_java_style_array(LEN);

    long long int start = current_timestamp();

    copy_java_style_arrays(source, dest, LEN);
    
    long long int end = current_timestamp();
    printf("%lld\n", end - start);
}

void measure_deep_copying_of_java_style_arrays() {
    Person** source = allocate_java_style_array(LEN);
    Person** dest = allocate_java_style_array(LEN);

    long long int start = current_timestamp();

    deep_copy_java_style_arrays(source, dest, LEN);
    
    long long int end = current_timestamp();
    printf("%lld\n", end - start);
}

void measure_copying_of_rust_style_arrays() {
    Person* source = allocate_rust_style_array(LEN);
    Person* dest = allocate_rust_style_array(LEN);

    long long int start = current_timestamp();

    copy_rust_style_arrays(source, dest, LEN);
    
    long long int end = current_timestamp();
    printf("%lld\n", end - start);
}

void measure_copying_of_c_style_arrays() {
    Person** source = allocate_c_style_array(LEN);
    Person** dest = allocate_c_style_array(LEN);

    long long int start = current_timestamp();

    copy_c_style_arrays(source, dest, LEN);
    
    long long int end = current_timestamp();
    printf("%lld\n", end - start);
}

int main() {
    measure_copying_of_java_style_arrays();
    measure_deep_copying_of_java_style_arrays();
    measure_copying_of_rust_style_arrays();
    measure_copying_of_c_style_arrays();
}
