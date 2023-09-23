#!/bin/bash -e
function bench_run_cpu {
	CPU=$1
	TOTAL_SIZE=$(echo "4 * 1024 * 1024" | bc)
	BLOCK_SIZE=2
	COUNT=$(echo "${TOTAL_SIZE} / ${BLOCK_SIZE}" | bc)
	CMD="dd if=/dev/urandom bs=${BLOCK_SIZE} count=${COUNT} | wc -c"
	RESULT=$(taskset -c ${CPU} /bin/bash -c "${CMD}" 2>&1 | grep copied)
	REALTIME=$(echo "${RESULT}" | cut -d ',' -f 3 | cut -d ' ' -f 2)
	echo "${CPU},${TOTAL_SIZE},${BLOCK_SIZE},${COUNT},${REALTIME}"
}
for ((CPU_IDX=0; CPU_IDX<$(nproc); CPU_IDX++))
do
	bench_run_cpu ${CPU_IDX}
done
