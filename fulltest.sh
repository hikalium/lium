#!/bin/bash -xe

# Build & Install
make
make install

# make a work dir
mkdir -p fulltest_workdir
WORKDIR=$(readlink -f ./fulltest_workdir)
echo ${WORKDIR}

export CROS_VERSION=R113-15384.67.0 # should be a publicly available version
export CROS=${WORKDIR}/chromiumos
lium sync --repo ${CROS} --version ${CROS_VERSION}
