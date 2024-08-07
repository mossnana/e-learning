import { hamiltonianCycle } from './hamiltonian-cycle.mjs'
import test from 'node:test'
import assert from 'node:assert/strict'

test('send graph to hamiltonianCycle', function() {
    const graph = [
        [0, 1, 0, 1, 0],
        [1, 0, 1, 1, 1],
        [0, 1, 0, 0, 1],
        [1, 0, 1, 0, 1],
        [0, 1, 1, 1, 0],
    ]
    const expected = [0,1,3,4,2]
    const actual = hamiltonianCycle(graph)
    assert.deepEqual(actual, expected, `result should be ${expected}, but got ${actual}`)
})
