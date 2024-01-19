import numpy as np
import json

from sklearn.linear_model import SGDRegressor

from .utils import ndarray_from_address


def ndarray_from_array2(arr):
    shape = (arr.dim1, arr.dim2)
    address = arr.data_address
    out = ndarray_from_address(address, "<f8", shape)
    # print(out[:5])
    return out


_models = {}


def new_model():
    global _models
    model = SGDRegressor(max_iter=10_000)
    key = id(model)
    _models[key] = model
    return key


def delete_model(model_key):
    global _models
    if model_key in _models:
        del _models[model_key]


def fit(model_key, x, y):
    global _models
    model = _models[model_key]
    y = y.reshape(-1)
    model.fit(x, y)
    print(model.coef_)
    print(model.intercept_)


def predict(model_key, x):
    global _models
    model = _models[model_key]
    return model.predict(x)


def get_params(model_key: int) -> str:
    global _models
    model = _models[model_key]
    params = {
        "coef": model.coef_.tolist(),
        "intercept": model.intercept_,
        "loss": model.loss,
    }
    return json.dumps(params)


def set_params(model_key: int, params: str):
    global _models
    model = _models[model_key]
    params = json.loads(params)
    model.coef_ = np.array(params["coef"], dtype=np.float64)
    model.intercept_ = params["intercept"]
