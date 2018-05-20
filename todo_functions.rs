pub fn checkConsistency(hash: List (Hash))
{
   let mut visited_hashes = HashSet::<Hash>::new();
   let mut diff = HashMap::<hash, f64>::new();
   
   
   for hash_param in hash.iter()
   {
    
	if(!update_diff(visited_hashes, diff, hash_param))
	  {
	     return false; 
	  }
   
   }
   return true;
}



pub fn init(&mut self) {
 let latestConsistentMilestone :MilestoneViewModel ;
 latestConsistentMilestone  =buildSnapshot();
 if(latestConsistentMilestone!=null )
   {
 
    debug!("Loaded consistent milestone:= {}",latestConsistentMilestone.index());
    milestone.latestSolidSubtangleMilestone = latestConsistentMilestone.getHash();
    milestone.latestSolidSubtangleMilestoneIndex = latestConsistentMilestone.index();
   }
    
}


pub fn update_diff(&mut self, approved_hashes: HashSet<Hash>, diff: HashMap<Address, i64>, tip:
    Hash) -> Result<bool, TransactionError> {
 
         if let Ok(mut hive) = self.hive.lock() {
             match hive.storage_load_transaction(&na_hash) {
                 Some(t) => {
                     if t.is_solid() {
                         return false;
                     }
                 },
                 None => return Err(TransactionError::InvalidHash)
             };
         }
        let mut visited_hashes = HashSet::<Hash>::new(approved_hashes);
		let mut currentState = getLatestDiff (visited_hashes,tip,milestone.latestSnapshot.index(),false);
		if(currentState==null )
		{
		retutn false;
		}
		
			for(key,value) in &*diff
		{
		   if (currentState.insert(key,((hash, aLong) -> value + aLong)) == null)
		       {
			      currentState.insert(key, value);
			   }
		}
		
        let is_consistent = snapshot.isConsistent(milestone.latestSnapshot.patchedDiff(currentState));
        if is_consistent {
		
		    diff.insert(currentState);
            approvedHashes.insert(visitedHashes);
        }
        Ok(is_consistent)
    }