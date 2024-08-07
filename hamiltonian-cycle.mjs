/*
จากตัวแปร graph อธิบายได้ว่า
ตัวแปรนี้เป็นตัวแทนของ graph 2 มิติโดย
ใน array ชั้นแรก ตำแหน่ง index จะหมายถึงหมายเลขกำกับ node นั้นๆ (vertex)
ใน array ชั้นที่สอง ตำแหน่ง index จะแสดงความสัมพันธ์ระหว่าง node นั้นๆกับ node อื่นๆ (0 คือ ไม่เชื่อมต่อ, 1 คือเชื่อมต่อ)

const graph = [
  [0, 1, 0, 1, 0], // โหนด 0 [0 คือไม่เชื่อมต่อกับตัวเอง, 1 คือเชื่อมต่อกับ node 1, 0 คือไม่เชื่อมต่อกับ node 2, 1 คือเชื่อมต่อกับ node 3, 0 คือไม่เชื่อมต่อกับ node 4]
  [1, 0, 1, 1, 1], // โหนด 1
  [0, 1, 0, 0, 1], // โหนด 2
  [1, 0, 1, 0, 1], // โหนด 3
  [0, 1, 1, 1, 0], // โหนด 4
]

วาดได้ออกมาเป็น

0 -- 1 -- 2
\  /   \  /
  3 --- 4
*/

export function hamiltonianCycle(graph) {
  const numberOfVertex = graph.length
  const visited = [0] // กำหนดให้เริ่มต้นจาก node 0 ก่อน (ซึ่งจริงๆไม่จำเป็น แค่จะได้ดูเข้าใจง่าย จริงๆ เริ่มจาก node ไหนก็ได้)

  function hamiltonianCycleUtil(visited) {
    // Base case in recursive function
    if (visited.length === numberOfVertex) {
      const lastVisitedNode = visited.at(-1) // .at(-1) คือหาเอาตัวสุดท้ายของ array นั้น มีค่าเท่ากับ .length-1
      return graph[lastVisitedNode][0] !== 0
    }
  
    for (let currentNode = 1; currentNode < numberOfVertex; currentNode++) {
      if (graph[visited.at(-1)][currentNode] === 0) {
        continue
      }
  
      if (visited.includes(currentNode)) {
        continue
      } else {
        visited.push(currentNode)
      }
  
      if (hamiltonianCycleUtil(visited)) {
        return true
      }
      visited.pop()
    }
    return false
  }

  return hamiltonianCycleUtil(visited) ? visited : null
}
