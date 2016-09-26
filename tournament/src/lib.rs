use std::collections::BTreeMap;

const INFORMATIONS_BY_LINE: usize = 3;
const WHITESPACES_FOR_NAME: usize = 30;

#[derive(Debug, PartialEq, Eq)]
struct TeamResults {
    name: String,
    drawn: usize,
    lost: usize,
    won: usize,
}

impl TeamResults {
    fn new(team_name: &str) -> Self {
        TeamResults {
            name: team_name.to_string(),
            drawn: 0,
            lost: 0,
            won: 0,
        }
    }

    fn drawn(&mut self) {
        self.drawn += 1;
    }

    fn lost(&mut self) {
        self.lost += 1;
    }

    fn won(&mut self) {
        self.won += 1;
    }

    fn compute_points(&self) -> usize {
        self.won * 3 + self.drawn
    }

    fn played(&self) -> usize {
        self.drawn + self.lost + self.won
    }

}

impl std::fmt::Display for TeamResults {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{name:<width_name$} |  {played} |  {won} |  {drawn} |  {lost} |  {total}", 
               name=(self.name),
               width_name=WHITESPACES_FOR_NAME,
               played=self.played(),
               won=self.won,
               drawn=self.drawn,
               lost=self.lost,
               total=self.compute_points())
    }
}

fn parse_line(input: &str) -> Vec<&str> {
    input.split(";").collect()
}

fn analyze_results_for(input: &str, team_table: &mut BTreeMap<String, TeamResults>) {
    let mut informations = parse_line(input);
    if informations.len() != INFORMATIONS_BY_LINE {
        return
    }
    match (informations.pop().unwrap(), informations.pop().unwrap(), informations.pop().unwrap()) {
        ("draw", team_a, team_b) => {
            {
                let mut results_for_team_a: &mut TeamResults = team_table.entry(team_a.to_string()).or_insert(TeamResults::new(team_a));
                results_for_team_a.drawn();
            }
            {
                let mut results_for_team_b: &mut TeamResults = team_table.entry(team_b.to_string()).or_insert(TeamResults::new(team_b));
                results_for_team_b.drawn();
            }
        },
        ("loss", team_w, team_l) | ("win", team_l, team_w) => {
            {
                let mut results_for_team_w: &mut TeamResults = team_table.entry(team_w.to_string()).or_insert(TeamResults::new(team_w));
                results_for_team_w.won();
            }
            {
                let mut results_for_team_l: &mut TeamResults = team_table.entry(team_l.to_string()).or_insert(TeamResults::new(team_l));
                results_for_team_l.lost();
            }
        },
        (_, _, _) => (),
    }
}

pub fn tally(input: &String) -> String {
    let input_string: Vec<&str> = (*input).split("\n").collect();
    let mut output_string = "Team                           | MP |  W |  D |  L |  P".to_string();
    let mut bmap: BTreeMap<String, TeamResults> = BTreeMap::new();
    for string in input_string {
        analyze_results_for(string, &mut bmap);
    }
    // Sort the hashmap by score, and transform this one as a Vector
    let mut bmap_vec: Vec<(&String, &TeamResults)> = bmap.iter().collect();
    bmap_vec.sort_by(|a, b| b.1.compute_points().cmp(&a.1.compute_points()));
    for (_, v) in bmap_vec.into_iter() {
        output_string = format!("{}\n{}", output_string, v);
    }
    output_string
}
