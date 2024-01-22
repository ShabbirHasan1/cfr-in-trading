import numpy as np
import json

from sklearn.linear_model import SGDRegressor, LinearRegression

from .utils import ndarray_from_address


def ndarray_from_array2(arr):
    shape = (arr.dim1, arr.dim2)
    address = arr.data_address
    out = ndarray_from_address(address, "<f8", shape)
    return out


_models = {}


def new_model():
    global _models
    model = LinearRegression()
    # model = SGDRegressor(max_iter=1_000)
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


def predict(model_key, x):
    global _models
    model = _models[model_key]
    prediction = model.predict(x)
    return prediction


def get_params(model_key: int) -> str:
    global _models
    model = _models[model_key]
    if not hasattr(model, "coef_") or model.coef_ is None:
        coef_ = []
    else:
        coef_ = model.coef_.tolist()
    if not hasattr(model, "intercept_") or model.intercept_ is None:
        intercept_ = 0
    elif isinstance(model.intercept_, np.ndarray):
        intercept_ = model.intercept_[0]
    else:
        intercept_ = model.intercept_
    params = {
        "coef": coef_,
        "intercept": intercept_,
        "loss": None,
    }
    s = json.dumps(params)
    return s


def set_params(model_key: int, params: str):
    global _models
    model = _models[model_key]
    params = json.loads(params)
    model.coef_ = np.array(params["coef"], dtype=np.float64)
    model.intercept_ = params["intercept"]
