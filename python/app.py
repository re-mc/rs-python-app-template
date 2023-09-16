"""
Rust / Python App Template
Author: AitherNight
"""

import rust
import os, sys
from time import sleep

# Import flexx (library used for the gui), if it isn't installed, install it
try:
    from flexx import flx
except ModuleNotFoundError:
    print('Flexx not found, installing...')
    if sys.platform not in ['linux', 'macos', 'other']:
        print('Windows detected.')
        os.system(f'python -m pip install flexx')
    else:
        print('Linux detected.')
        os.system(f'python3 -m pip install flexx')
    print('Flexx installed, please run the program again.')
    sleep(0.5)
    exit(0)


flx.assets.associate_asset(__name__, 'https://cdnjs.cloudflare.com/ajax/libs/materialize/1.0.0/css/materialize.min.css')
flx.assets.associate_asset(__name__, 'https://cdnjs.cloudflare.com/ajax/libs/materialize/1.0.0/js/materialize.min.js')


class PythonApp(flx.PyWidget):
    def init(self):
        super().init()
        self.rust_version = flx.Label(text=f'Application Version: {str(rust.get_pkg_ver())}')
        self.example_button = flx.Button(text='Click Me!')



def main():
    app = flx.App(PythonApp, title='Rust-Python App', icon='https://rust-lang.org/logos/rust-logo-512x512.png')
    app.launch('app')

    flx.run()
