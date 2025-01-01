mod graph;

fn main() {
  let problem = vec![
    vec![0, 5, -1, 1, -1],
    vec![-1, 0, 2, -1, 6],
    vec![-1, -1, 0, -1, 3],
    vec![-1, 2, -1, 0, -1],
    vec![-1, -1, -1, 4, 0],
  ];
  let problem = vec![
    vec![0, 523, 345, -1, -1, -1],
    vec![523, 0, 200, 548, -1, -1],
    vec![345, 200, 0, 360, 467, -1],
    vec![-1, 548, 360, 0, 245, 320],
    vec![-1, -1, 467, 245, 0, 555],
    vec![-1, -1, -1, 320, 555, 0],
  ];
  graph::floyd_warshall::floyd_warshall(problem);
}
