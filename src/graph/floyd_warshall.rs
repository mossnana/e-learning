pub fn print_table(title: String, data: Vec<Vec<i32>>) {
  let len = data.len();
  println!("\nTable {title}");
  for i in 0..len {
    let column = i + 1;
    print!("\t{column}");
  }
  print!("\n");
  for i in 0..len {
    let row = i + 1;
    print!("{row}");
    for vertex in data[i].iter() {
      let format_vertex = match vertex {
        -1 => "∞".to_string(),
        _ => format!("{vertex}").to_string(),
      };
      print!("\t{format_vertex}");
    }
    print!("\n");
  }
}

// Floyd Warshall Table
pub fn floyd_warshall(adjacency_matrix: Vec<Vec<i32>>) {
  print_table(String::from("Adjacency Matrix"), adjacency_matrix.clone());

  let vertex_no = adjacency_matrix.len();
  let mut distances = adjacency_matrix.clone();

  /*
    วนหาทุก vertex ว่าใน [i] -> [k] -> [j] เป็นระยะทางที่น้อยกว่า [i] -> [j] นั้นหรือไม่
  */

  // k คือ vertex กลาง
  for k in 0..vertex_no {
    // i คือ vertex ต้นทาง
    for i in 0..vertex_no {
      // j คือ vertex ปลายทาง
      for j in 0..vertex_no {
        let intermediated_node_distance = match distances[i][k] == -1 || distances[k][j] == -1 {
          true => i32::MAX,
          false => distances[i][k] + distances[k][j],
        };

        let direct_distance = match distances[i][j] == -1 {
          true => i32::MAX,
          false => distances[i][j],
        };

        if intermediated_node_distance < direct_distance {
          distances[i][j] = intermediated_node_distance;
        }
      }
    }
  }

  print_table("Floyd Warshall".to_string(), distances);
}
