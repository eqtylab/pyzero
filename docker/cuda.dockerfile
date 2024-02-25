# syntax = devthefuture/dockerfile-x

INCLUDE ./docker/base.dockerfile

# Install CUDA dev kit
ARG CUDA_KEYRING='cuda-keyring_1.1-1_all.deb'
ARG CUDA_TOOLKIT='cuda-toolkit-12-3'

RUN curl -o "${CUDA_KEYRING}" "https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64/${CUDA_KEYRING}"
RUN dpkg -i cuda-keyring_1.1-1_all.deb
RUN apt-get update
RUN apt-get --yes install ${CUDA_TOOLKIT}
ENV PATH=/usr/local/cuda/bin:$PATH
