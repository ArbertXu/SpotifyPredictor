


#[derive(Debug)]
pub enum TreeNode {
    Leaf {prediction: u8},
    Decision {
        feature_index: usize,
        threshold: f32,
        left: Box<TreeNode>,
        right: Box<TreeNode>,
    },
}

fn gini_impurty(labels: &[u8]) ->f32 {
    let mut counts = [0.0, 0.0];
    let total = labels.len() as f32;
    for &label in labels {
        counts[label as usize] += 1.0;
    }
    let mut impurity = 1.0;
    for &count in &counts {
        let p = count / total;
        impurity -= p * p;
    }
    impurity
}

fn find_split(features: &[Vec<f32>], labels: &[u8], indices: &[usize],) -> Option<(usize, f32, Vec<usize>, Vec<usize>)> {
    println!("Features: {}, Labels: {}", features.len(), labels.len());
    let n_features = features[0].len();
    let mut best_gini_impurity = f32::MAX;
    let mut best_split = None;
    let mut left_indices = Vec::with_capacity(indices.len());
    let mut right_indices = Vec::with_capacity(indices.len());
    let mut left_labels = Vec::with_capacity(indices.len());
    let mut right_labels = Vec::with_capacity(indices.len());
    for feature_index in 0..n_features{
        let mut sorted: Vec<(usize, f32)> = indices.iter().map(|&i| (i, features[i][feature_index])).collect();
        sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        for i in 1..sorted.len() - 1 {
            let threshold = (sorted[i-1].1 + sorted[i].1) / 2.0;
            left_indices.clear();
            right_indices.clear();
            left_labels.clear();
            right_labels.clear();
            for (idx, val) in &sorted {
                if *val <= threshold {
                    left_indices.push(*idx);
                    left_labels.push(labels[*idx]);
                } else {
                    right_indices.push(*idx);
                    right_labels.push(labels[*idx]);
                }
            }
            if left_labels.is_empty() || right_labels.is_empty() {
                continue;
            }
            let gini = (left_labels.len() as f32 * gini_impurty(&left_labels) + 
                        right_labels.len() as f32 * gini_impurty(&right_labels)) / labels.len() as f32;
            if gini < best_gini_impurity {
                best_gini_impurity = gini;
                best_split = Some((feature_index, threshold, left_indices.clone(),
                                    right_indices.clone(),));
            }
        }
    }
    best_split
}

pub fn build_tree(features: &[Vec<f32>], labels: &[u8], indices: &[usize], depth: usize, max_depth: usize) -> TreeNode {
    let mut num_ones = 0;
    for &i in indices {
        if labels[i] == 1 { 
            num_ones += 1;
        }
    }
    let num_zeroes = indices.len() - num_ones;
    if depth >= max_depth || num_ones == 0 || num_zeroes == 0 {
        let prediction = if num_ones >= num_zeroes {1} else {0};
        return TreeNode::Leaf {prediction};
    }
    if let Some((feature_index, threshold, left_indicies, right_indicies)) = find_split(features, labels, indices) {
        
        if left_indicies.is_empty() || right_indicies.is_empty() {
        let prediction = if num_ones >= num_zeroes {1} else {0};
        return TreeNode::Leaf {prediction};
        }
        TreeNode::Decision {
        feature_index,
        threshold,
        left: Box::new(build_tree(&features, &labels, &left_indicies, depth + 1, max_depth)),
        right: Box::new(build_tree(&features, &labels, &right_indicies, depth + 1, max_depth)),
        }   
    } else {
        let prediction = if num_ones >= num_zeroes {1} else {0};
        TreeNode::Leaf {prediction}
    }
}

fn predict(tree: &TreeNode, sample: &[f32]) -> u8 {
    match tree {
        TreeNode::Leaf {prediction} => *prediction,
        TreeNode::Decision {
            feature_index,
            threshold,
            left,
            right,
        } => {
            if sample[*feature_index] <= *threshold {
                predict(left, sample)
            } else {
                predict(right, sample)
            }
        }
    }
}

pub fn accuracy(tree: &TreeNode, features: &[Vec<f32>], labels: &[u8]) -> f32 {
    // println!("Labels: {:?}", labels);
    let correct = features.iter().zip(labels).filter(|(x, y)| predict(tree,x) == **y).count();
    correct as f32 / labels.len() as f32
}