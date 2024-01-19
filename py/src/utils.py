import numpy as np


def ndarray_from_address(pointer, typestr, shape, copy=False, read_only_flag=False):
    """Generates numpy array from memory address
    https://docs.scipy.org/doc/numpy-1.13.0/reference/arrays.interface.html

    Parameters
    ----------
    pointer : int
        Memory address

    typestr : str
        A string providing the basic type of the homogenous array The
        basic string format consists of 3 parts: a character
        describing the byteorder of the data (<: little-endian, >:
        big-endian, |: not-relevant), a character code giving the
        basic type of the array, and an integer providing the number
        of bytes the type uses.

        The basic type character codes are:

        - t Bit field (following integer gives the number of bits in the bit field).
        - b Boolean (integer type where all values are only True or False)
        - i Integer
        - u Unsigned integer
        - f Floating point
        - c Complex floating point
        - m Timedelta
        - M Datetime
        - O Object (i.e. the memory contains a pointer to PyObject)
        - S String (fixed-length sequence of char)
        - U Unicode (fixed-length sequence of Py_UNICODE)
        - V Other (void * – each item is a fixed-size chunk of memory)

        See https://docs.scipy.org/doc/numpy-1.13.0/reference/arrays.interface.html#__array_interface__

    shape : tuple
        Shape of array.

    copy : bool
        Copy array.  Default False

    read_only_flag : bool
        Read only array.  Default False.
    """
    buff = {'data': (pointer, read_only_flag),
            'typestr': typestr,
            'shape': shape}

    class numpy_holder():
        pass

    holder = numpy_holder()
    holder.__array_interface__ = buff
    return np.array(holder, copy=copy)
