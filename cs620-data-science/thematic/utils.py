import os
import logging

DATA_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', 'data'))

def setup_logging():
    """
    Initialize logging system at desired level
    """
    logging.basicConfig(level=logging.INFO,
                        format='%(asctime)s %(levelname)-8s %(message)s',
                        datefmt='%m-%d %H:%M')
    console = logging.StreamHandler()
    console.setLevel(logging.INFO)
    formatter = logging.Formatter('%(name)-12s: %(levelname)-8s %(message)s')
    console.setFormatter(formatter)
    logging.getLogger('').addHandler(console)