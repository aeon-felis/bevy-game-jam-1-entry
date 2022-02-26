from omnipytent import *
from omnipytent.ext.idan import *


@task
def check(ctx):
    cargo['check', '-q'] & ERUN.bang


@task
def build(ctx):
    cargo['build', '-q'][
        '--features', 'bevy/dynamic',
    ]& ERUN.bang


@task
def run(ctx):
    cargo['run'][
        '--features', 'bevy/dynamic',
    ].with_env(
        RUST_LOG='pogo_hurdles=info',
        RUST_BACKTRACE='1',
    ) & BANG


@task
def test(ctx):
    cargo['test'].with_env(RUST_LOG='app=debug') & BANG


@task
def clean(ctx):
    cargo['clean'] & BANG
