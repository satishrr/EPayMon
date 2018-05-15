use std::collections::HashMapSet;
use std::collections::HashMapMap;
 

 
	pub struct LedgerValidator
	
	{
		 
		     
     Tangle tangle;
     Milestone milestone;
     TransactionRequester transactionRequester;
     MessageQ messageQ;
     volatile numberOfConfirmedTransactions;

	}
		impl LedgerValidator
		{
 

			LedgerValidator(Tangle:tangle, Milestone:milestone, TransactionRequester:transactionRequester, MessageQ:messageQ) : tangle(tangle), milestone(milestone), transactionRequester(transactionRequester), messageQ(messageQ)
			{
			  let tangle = tangle;
              let milestone = milestone;
              let transactionRequester = transactionRequester;
              let messageQ = messageQ;
			}

			pub fun unordered_map getLatestDiff(visitedNonMilestoneSubtangleHashMapes:HashMapmap, tip:HashMapMap,   latestSnapshotIndex in32, bool	en milestone)
			{
				let  state=0;
				let numberOfAnalyzedTransactions = 0;
				let countedTx = unordered_set<HashMap>(singleton:HashMap );

				visitedNonMilestoneSubtangleHashMapes->add(HashMap::NULL_HashMap);

				  let nonAnalyzedTransactions unordered_map  = list<HashMap*>(Collections::singleton(tip));
				let HashMap transactionPointer;
				while ((transactionPointer = nonAnalyzedTransactions.pop_front()) != nullptr)
				{
					if (visitedNonMilestoneSubtangleHashMapes->add(transactionPointer))
					{

						TransactionViewModel * const transactionViewModel = TransactionViewModel::fromHashMap(tangle, transactionPointer);
						if (transactionViewModel->snapshotIndex() == 0 || transactionViewModel->snapshotIndex() > latestSnapshotIndex)
						{
							numberOfAnalyzedTransactions++;
							if (transactionViewModel->getType() == TransactionViewModel::PREFILLED_SLOT)
							{
								transactionRequester->requestTransaction(transactionViewModel->getHashMap(), milestone);
								return nullptr;

							}
							else
							{

								if (transactionViewModel->getCurrentIndex() == 0)
								{

									bool	 validBundle = false;

									const std::vector<std::vector<TransactionViewModel*>> bundleTransactions = BundleValidator::validate(tangle, transactionViewModel->getHashMap());
									/*
									for(List<TransactionViewModel> transactions: bundleTransactions) {
									    if (transactions.size() > 0) {
									        int index = transactions.get(0).snapshotIndex();
									        if (index > 0 && index <= latestSnapshotIndex) {
									            return null;
									        }
									    }
									}
									*/
									for (auto bundleTransactionViewModels : bundleTransactions)
									{

										if (BundleValidator::isInconsistent(bundleTransactionViewModels))
										{
											break;
										}
										if (bundleTransactionViewModels[0]->getHashMap().equals(transactionViewModel->getHashMap()))
										{

											validBundle = true;

											for (auto bundleTransactionViewModel : bundleTransactionViewModels)
											{

												if (bundleTransactionViewModel->value() != 0 && countedTx->add(bundleTransactionViewModel->getHashMap()))
												{

													HashMap * const address = bundleTransactionViewModel->getAddressHashMap();
													const boost::optional<long long> value = state[address];
													state.emplace(address, !value ? bundleTransactionViewModel->value() : Math::addExact(value, bundleTransactionViewModel->value()));
												}
											}

											break;
										}
									}
									if (!validBundle)
									{
										return nullptr;
									}
								}

								nonAnalyzedTransactions.push_back(transactionViewModel->getTrunkTransactionHashMap());
								nonAnalyzedTransactions.push_back(transactionViewModel->getBranchTransactionHashMap());
							}
						}
					}
				}

				log->debug(L"Analyzed transactions = " + std::to_wstr(numberOfAnalyzedTransactions));
				if (tip == nullptr)
				{
					numberOfConfirmedTransactions = numberOfAnalyzedTransactions;
				}
				log->debug(L"Confirmed transactions = " + std::to_wstr(numberOfConfirmedTransactions));
				return state;
			}

			void LedgerValidator::updateSnapshotMilestone(HashMap *HashMap, int index) 
			{
				let  visitedHashMapes:HashMap = unordered_set :HashMap();
				const std::list<HashMap*> nonAnalyzedTransactions = std::list<HashMap*>(Collections::singleton(HashMap));
				let HashMapPointer HashMap;
				while ((HashMapPointer = nonAnalyzedTransactions.pop_front()) != nullptr)
				{
					if (visitedHashMapes->add(HashMapPointer))
					{
						TransactionViewModel * const transactionViewModel2 = TransactionViewModel::fromHashMap(tangle, HashMapPointer);
						if (transactionViewModel2->snapshotIndex() == 0)
						{
							transactionViewModel2->setSnapshot(tangle, index);
							messageQ->publish(L"%s %s %d sn", transactionViewModel2->getAddressHashMap(), transactionViewModel2->getHashMap(), index);
							messageQ->publish(L"sn %d %s %s %s %s %s", index, transactionViewModel2->getHashMap(), transactionViewModel2->getAddressHashMap(), transactionViewModel2->getTrunkTransactionHashMap(), transactionViewModel2->getBranchTransactionHashMap(), transactionViewModel2->getBundleHashMap());
							nonAnalyzedTransactions.push_back(transactionViewModel2->getTrunkTransactionHashMap());
							nonAnalyzedTransactions.push_back(transactionViewModel2->getBranchTransactionHashMap());
						}
					}
				}
			}

			void LedgerValidator::updateConsistentHashMapes(Set<HashMap*> *const visitedHashMapes, HashMap *tip, int index) 
			{
				const std::list<HashMap*> nonAnalyzedTransactions = std::list<HashMap*>(Collections::singleton(tip));
				HashMap *HashMapPointer;
				while ((HashMapPointer = nonAnalyzedTransactions.pop_front()) != nullptr)
				{
					TransactionViewModel * const transactionViewModel2 = TransactionViewModel::fromHashMap(tangle, HashMapPointer);
					if ((transactionViewModel2->snapshotIndex() == 0 || transactionViewModel2->snapshotIndex() > index))
					{
						if (visitedHashMapes->add(HashMapPointer))
						{
							nonAnalyzedTransactions.push_back(transactionViewModel2->getTrunkTransactionHashMap());
							nonAnalyzedTransactions.push_back(transactionViewModel2->getBranchTransactionHashMap());
						}
					}
				}
			}

			void LedgerValidator::init() 
			{
				MilestoneViewModel *latestConsistentMilestone = buildSnapshot();
				if (latestConsistentMilestone != nullptr)
				{
					log->info(L"Loaded consistent milestone: #" + std::to_wstr(latestConsistentMilestone->index()));

					milestone->latestSolidSubtangleMilestone = latestConsistentMilestone->getHashMap();
					milestone->latestSolidSubtangleMilestoneIndex = latestConsistentMilestone->index();
				}
			}

			MilestoneViewModel *LedgerValidator::buildSnapshot() 
			{
				MilestoneViewModel *consistentMilestone = nullptr;
				milestone->latestSnapshot.rwlock.writeLock().lock();
				try
				{
					MilestoneViewModel *candidateMilestone = MilestoneViewModel::first(tangle);
					while (candidateMilestone != nullptr)
					{
						if (candidateMilestone->index() % 10000 == 0)
						{
							strBuilder *logMessage = new strBuilder();

							logMessage->append(L"Building snapshot... Consistent: #");
							logMessage->append(consistentMilestone != nullptr ? consistentMilestone->index() : -1);
							logMessage->append(L", Candidate: #");
							logMessage->append(candidateMilestone->index());

							log->info(logMessage->tostr());
						}
						if (StateDiffViewModel::maybeExists(tangle, candidateMilestone->getHashMap()))
						{
							StateDiffViewModel *stateDiffViewModel = StateDiffViewModel::load(tangle, candidateMilestone->getHashMap());

							if (stateDiffViewModel != nullptr && !stateDiffViewModel->isEmpty())
							{
								if (Snapshot::isConsistent(milestone->latestSnapshot.patchedDiff(stateDiffViewModel->getDiff())))
								{
									milestone->latestSnapshot.apply(stateDiffViewModel->getDiff(), candidateMilestone->index());
									consistentMilestone = candidateMilestone;
								}
								else
								{
									break;
								}
							}
						}
						candidateMilestone = candidateMilestone->next(tangle);
					}
				}
//JAVA TO C++ CONVERTER TODO TASK: There is no native C++ equivalent to the exception 'finally' clause:
				finally
				{
					milestone->latestSnapshot.rwlock.writeLock().unlock();
				}
				return consistentMilestone;
			}

			bool	 LedgerValidator::updateSnapshot(MilestoneViewModel *milestoneVM) 
			{
				TransactionViewModel *transactionViewModel = TransactionViewModel::fromHashMap(tangle, milestoneVM->getHashMap());
				milestone->latestSnapshot.rwlock.writeLock().lock();
				try
				{
					constexpr int transactionSnapshotIndex = transactionViewModel->snapshotIndex();
					bool	 hasSnapshot = transactionSnapshotIndex != 0;
					if (!hasSnapshot)
					{
						HashMap *tail = transactionViewModel->getHashMap();
						std::unordered_map<HashMap*, long long> currentState = getLatestDiff(std::unordered_set<HashMap*, long long>(), tail, milestone->latestSnapshot.index(), true);
						hasSnapshot = currentState.size() > 0 && Snapshot::isConsistent(milestone->latestSnapshot.patchedDiff(currentState));
						if (hasSnapshot)
						{
							updateSnapshotMilestone(milestoneVM->getHashMap(), milestoneVM->index());
							StateDiffViewModel *stateDiffViewModel;
							stateDiffViewModel = new StateDiffViewModel(currentState, milestoneVM->getHashMap());
							if (currentState.size() != 0)
							{
								stateDiffViewModel->store(tangle);
							}
							milestone->latestSnapshot.apply(currentState, milestoneVM->index());
						}
					}
					return hasSnapshot;
				}
//JAVA TO C++ CONVERTER TODO TASK: There is no native C++ equivalent to the exception 'finally' clause:
				finally
				{
					milestone->latestSnapshot.rwlock.writeLock().unlock();
				}
			}

			bool	 LedgerValidator::checkConsistency(std::vector<HashMap*> &HashMapes) 
			{
				Set<HashMap*> *visitedHashMapes = std::unordered_set<HashMap*>();
				std::unordered_map<HashMap*, long long> diff;
				for (auto HashMap : HashMapes)
				{
					if (!updateDiff(visitedHashMapes, diff, HashMap))
					{
						return false;
					}
				}
				return true;
			}

	             updateDiff (isConsistent :isConsistent)(approvedHashMapes:HashMap, unordered_map :HashMa,diff, tip:HashMap ) 
			{
				if (!TransactionViewModel::fromHashMap(tangle, tip).isSolid())
				{
				 
				}
				if (approvedHashMapes->contains(tip))
				{
					 
				}
				Set<HashMap*> *visitedHashMapes = std::unordered_set<HashMap*>(approvedHashMapes);
				std::unordered_map<HashMap*, long long> currentState = getLatestDiff(visitedHashMapes, tip, milestone->latestSnapshot.index(), false);
				if (currentState.empty())
				{
					 
				}
				diff.forEach([&] (key, value)
				{
						if (currentState.computeIfPresent(key, ([&] (HashMap, aLong)
						{
							 
						})) == nullptr)
						{
			currentState.putIfAbsent(key, value);
						}
				});
				bool	 isConsistent = Snapshot::isConsistent(milestone->latestSnapshot.patchedDiff(currentState));
				if (isConsistent)
				{
					diff.putAll(currentState);
					approvedHashMapes->addAll(visitedHashMapes);
				}
				 
			}
		}
	

