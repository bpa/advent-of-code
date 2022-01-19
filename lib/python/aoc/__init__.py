from collections import defaultdict
import logging
from logging import debug, info, warning, error
import itertools
from functools import reduce
import math
from math import ceil
import re
import os
import sys
if os.environ.get('DEBUG', False):
    logging.basicConfig(level=logging.DEBUG,
                        format='%(filename)s:%(lineno)s %(message)s')
else:
    logging.basicConfig(level=logging.WARNING,
                        format='%(filename)s:%(lineno)s %(message)s')
from .grid import Grid
from .main import main
from .point import Point
from .util import *
from .string import *
