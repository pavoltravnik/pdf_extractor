# pdf_extractor


## Use

```python
import ctypes

path_pdf = ''
path_txt = ''

# Load the shared library
lib = ctypes.CDLL('./target/release/libpdf_extractor.dylib')

# Define the argument and return types
lib.extract_text_from_pdf.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
lib.extract_text_from_pdf.restype = ctypes.c_char_p

def extract_text_from_pdf(path_pdf_clone, path_txt):
    result = lib.extract_text_from_pdf(path_pdf_clone.encode('utf-8'), path_txt.encode('utf-8'))
    return result.decode('utf-8')


extract_text_from_pdf(path_pdf, path_txt)

```