import logging
import os

if os.environ.get("DEBUG", False):
    logging.basicConfig(
        level=logging.DEBUG, format="%(filename)s:%(lineno)s %(message)s"
    )
else:
    logging.basicConfig(
        level=logging.WARNING, format="%(filename)s:%(lineno)s %(message)s"
    )


def debug(*args):
    logging.debug(" ".join([str(a) for a in args]), stacklevel=3)


def error(*args):
    logging.error(" ".join([str(a) for a in args]), stacklevel=3)


def info(*args):
    logging.info(" ".join([str(a) for a in args]), stacklevel=3)


def warn(*args):
    logging.warn(" ".join([str(a) for a in args]), stacklevel=3)


def warning(*args):
    logging.warning(" ".join([str(a) for a in args]), stacklevel=3)
