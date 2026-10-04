#!/bin/bash
git clone https://github.com/obra/superpowers.git .task-scratch/superpowers
cd .task-scratch/superpowers
git rev-parse HEAD > ../superpowers-revision.txt
