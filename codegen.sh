#!/bin/bash
while read protocol; do
	python ./py2rs.py ../s2protocol-python/protocol${protocol}.py > src/protocol/protocol${protocol}_def.rs
done < protocols.txt
