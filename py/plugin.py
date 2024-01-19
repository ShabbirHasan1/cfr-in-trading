import cffi
import os

py_root = os.path.abspath(f"__file__/..")

ffibuilder = cffi.FFI()

with open(f"{py_root}/plugin.h") as f:
    text = f.read()
    text = "".join(line for line in text.splitlines() if not line.startswith("#"))
    ffibuilder.embedding_api(text)

ffibuilder.set_source("pyemb_plugin", """
#include "plugin.h"
""")

ffibuilder.embedding_init_code(f"""
    import sys
    sys.path.insert(0, "{py_root}")
    from pyemb_plugin import ffi
    
    @ffi.def_extern()
    def new_model() -> int:
        import src
        model_key: int = src.new_model()
        return model_key
    
    @ffi.def_extern()
    def delete_model(model_key: int):
        import src
        src.delete_model(model_key)

    @ffi.def_extern()
    def fit(model_key: int, x: ffi.CData, y: ffi.CData):
        import src
        x = src.ndarray_from_array2(x)
        y = src.ndarray_from_array2(y)
        src.fit(model_key, x, y)
        
    @ffi.def_extern()
    def predict(output: ffi.CData, model_key: int, x: ffi.CData):
        import src
        x = src.ndarray_from_array2(x)
        output = src.ndarray_from_array2(output)
        output[:, :] = src.predict(model_key, x)
    
    @ffi.def_extern()
    def get_params(model_key: int) -> ffi.CData:
        import src
        output = src.get_params(model_key)
        output = ffi.new("char[]", output.encode())
        return output
    
    @ffi.def_extern()
    def set_params(model_key: int, params: str):
        import src
        return src.set_params(model_key, params)
""")

ffibuilder.compile(target="libpyemb.*", verbose=True)
