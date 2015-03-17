from distutils.core import setup
from Cython.Build import cythonize
from distutils.extension import Extension

sourcefiles = ['pyshn.pyx', 'shn.c']
extensions = [Extension('pyshn', sourcefiles)]


setup(
    ext_modules = cythonize(extensions)
)

