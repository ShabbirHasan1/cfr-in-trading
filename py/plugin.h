#include <stdint.h>

struct Array2Ptr {
    uint64_t data_address;
    int dim1;
    int dim2;
};

uint64_t new_model();

void delete_model(uint64_t model_key);

void fit(uint64_t model_key, struct Array2Ptr x, struct Array2Ptr y);

void predict(struct Array2Ptr output, uint64_t model_key, struct Array2Ptr x);

void get_params(uint64_t model_key, char* output);

void set_params(uint64_t model_key, const char* params);
