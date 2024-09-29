#!/usr/bin/env bash

systemctl list-unit-files --type service --state running  --all --no-legend  | awk '{print $1}' > /tmp/services-running
systemctl list-unit-files --type service --state disabled --all  --no-legend  | awk '{print $1}' > /tmp/services-disabled
systemctl list-unit-files --type socket --state running  --all --no-legend  | awk '{print $1}' > /tmp/sockets-running
systemctl list-unit-files --type socket --state disabled --all --no-legend  | awk '{print $1}' > /tmp/sockets-disabled