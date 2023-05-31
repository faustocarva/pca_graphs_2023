use std::collections::{VecDeque};

/// Max flow algorithms.

// Implementação do algoritmo de Edmonds-Karp para encontrar o fluxo máximo em um grafo
pub fn edmonds_karp(graph: &Vec<Vec<i32>>, source: usize, sink: usize) -> i32 {
    let num_vertices = graph.len();
    let mut residual_graph = graph.clone();
    let mut parent = vec![None; num_vertices]; // Vetor que armazena o pai de cada vértice no augmenting path
    
    // Para debug - imprime o grafo inicial
    println!("Grafo inicial:");
    for k in 0..num_vertices {
        println!("{:?}", residual_graph[k]);
    }
    let mut max_flow = 0; // Inicializa o fluxo máximo com 0.

    // Enquanto houver um augmenting path no grafo residual
    while bfs(&residual_graph, source, sink, &mut parent) {
        let mut path_flow = i32::MAX; // Define o valor inicial para o fluxo mínimo do augmenting path como infinito

        // Encontrar o fluxo mínimo ao longo do augmenting path encontrado
        let mut v = sink; // Começa pelo vértice de destino
        while v != source {
            let u = parent[v].unwrap(); // Obtém o pai
            path_flow = path_flow.min(residual_graph[u][v]); //Atualiza o fluxo mínimo
            v = u; // Avança para o próximo vértice do augmenting path
        }

        // Atualizar as capacidades residuais do grafo residual
        v = sink;  // Começa pelo vértice de destino
        while v != source {
            let u = parent[v].unwrap();  // Obtém o pai
            residual_graph[u][v] -= path_flow; // Reduz a capacidade residual da aresta original            
            residual_graph[v][u] += path_flow; // Aumenta a capacidade residual da aresta reversa
            v = u; // Avança para o próximo vértice do augmenting path
        }

        // Imprime o grafo residual (mostra as capacidades restantes de cada aresta - normal e reversa)
        println!("Grafo residual a cada iteração:");
        for k in 0..num_vertices {
        println!("{:?}", residual_graph[k]);
    }
        // Incrementar o fluxo máximo
        max_flow += path_flow;
    }
    
    // Imprime o grafo residual (mostra as capacidades restantes de cada aresta - normal e reversa)
    println!("");
    println!("Grafo residual final:");
    for k in 0..num_vertices {
        println!("{:?}", residual_graph[k]);
    }
    max_flow // Retorna o fluxo máximo
}

// Busca em largura (BFS) para encontrar augmenting paths
pub fn bfs(graph: &Vec<Vec<i32>>, source: usize, sink: usize, parent: &mut Vec<Option<usize>>) -> bool {
    let num_vertices = graph.len();
    let mut visited = vec![false; num_vertices];

    let mut queue = VecDeque::new();
    queue.push_back(source);
    visited[source] = true;
    parent[source] = None;

    while !queue.is_empty() {
        let u = queue.pop_front().unwrap();
        for v in 0..num_vertices {
            // Só executa se o vértice não tiver sido visitado e a capacidade da aresta for maior que 0
            if !visited[v] && graph[u][v] > 0 {
                visited[v] = true;
                parent[v] = Some(u);
                queue.push_back(v);
            }
        } 
    }

    visited[sink]
}

#[cfg(test)]
mod test_flow {
    use crate::max_flow::edmonds_karp;
    
    #[test]
    fn test_flow_edmondskarp_preto_ppt() {
        // Grafo de exemplo
        let graph = vec![
            vec![0, 10, 10, 0, 0, 0],
            vec![0, 0, 0, 25, 0, 0],
            vec![0, 0, 0, 0, 15, 0],
            vec![0, 0, 0, 0, 0, 10],
            vec![0, 6, 0, 0, 0, 10],
            vec![0, 0, 0, 0, 0, 0],
        ];

        let source = 0;
        let sink = 5;

        let max_flow = edmonds_karp(&graph, source, sink);

        println!("Fluxo máximo: {}", max_flow);
    }

    #[test]
    fn test_flow_edmondskarp_losango() {
        // Grafo de exemplo
        let graph = vec![
            vec![0, 100, 100, 0],
            vec![0, 0, 1, 100],
            vec![0, 0, 0, 100],
            vec![0, 0, 0, 0],
        ];

        let source = 0;
        let sink = 3;

        let max_flow = edmonds_karp(&graph, source, sink);

        println!("Fluxo máximo: {}", max_flow);
    }

    #[test]
    fn test_flow_edmondskarp_livro() {
        // Grafo igual ao do livro do Cormen
        let graph = vec![
            vec![0, 16, 13, 0, 0, 0],
            vec![0, 0, 0, 12, 0, 0],
            vec![0, 4, 0, 0, 14, 0],
            vec![0, 0, 9, 0, 0, 20],
            vec![0, 0, 0, 7, 0, 4],
            vec![0, 0, 0, 0, 0, 0],
        ];

        let source = 0;
        let sink = 5;

        let max_flow = edmonds_karp(&graph, source, sink);

        println!("Fluxo máximo: {}", max_flow);
    }

    #[test]
    fn test_flow_edmondskarp2() {
        // Grafo de exemplo
        let graph = vec![
            vec![0, 22, 0, 4, 0, 0],
            vec![0, 0, 20, 7, 0, 0],
            vec![0, 0, 0, 0, 0, 15],
            vec![0, 0, 0, 0, 18, 0],
            vec![0, 0, 13, 0, 0, 20],
            vec![0, 0, 0, 0, 0, 0],
        ];

        let source = 0;
        let sink = 5;

        let max_flow = edmonds_karp(&graph, source, sink);

        println!("Fluxo máximo: {}", max_flow);
    }
}