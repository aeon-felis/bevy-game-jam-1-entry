from omnipytent import *
from omnipytent.ext.idan import *


@task
def check(ctx):
    cargo['check', '-q'] & ERUN.bang


@task
def build(ctx):
    cargo['build', '-q'] & ERUN.bang


@task
def run(ctx):
    cargo['run'].with_env(
        RUST_LOG='app=debug',
        RUST_BACKTRACE='1',
    ) & BANG


@task
def test(ctx):
    cargo['test'].with_env(RUST_LOG='app=debug') & BANG


@task
def clean(ctx):
    cargo['clean'] & BANG
