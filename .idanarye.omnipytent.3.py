from omnipytent import *
from omnipytent.ext.idan import *


@task
def check(ctx):
    cargo['check', '-q'] & ERUN.bang


@task
def build(ctx):
    cargo['build'][
        '--features', 'bevy/dynamic',
    ] & TERMINAL_PANEL.size(20)


@task
def run(ctx):
    cargo['run'][
        '--features', 'bevy/dynamic',
    ].with_env(
        RUST_LOG='pogo_hurdling=info',
        RUST_BACKTRACE='1',
    ) & TERMINAL_PANEL.size(20)


@task
def test(ctx):
    cargo['test'].with_env(RUST_LOG='app=debug') & BANG


@task
def clean(ctx):
    cargo['clean'] & BANG


@task
def launch_wasm(ctx):
    cargo['run'][
        '--target', 'wasm32-unknown-unknown'
    ].with_env(
        RUST_BACKTRACE='1',
    ) & TERMINAL_PANEL.size(20)
