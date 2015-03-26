from distutils.core import setup
from Cython.Build import cythonize
from distutils.extension import Extension

sourcefiles = ['decoder.pyx']
extensions = [Extension('decoder', sourcefiles, libraries=["vorbisfile"])]


setup(
    ext_modules = cythonize(extensions)
)

