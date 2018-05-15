//========================================================================
// This conversion was produced by the Free Edition of
// Java to C++ Converter courtesy of Tangible Software Solutions.
// Order the Premium Edition at https://www.tangiblesoftwaresolutions.com
//========================================================================

#include "LedgerValidator.h"

namespace com
{
	namespace iota
	{
		namespace iri
		{
			using namespace com::iota::iri::controllers;
			using com::iota::iri::model::Hash;
			using com::iota::iri::network::TransactionRequester;
			using com::iota::iri::zmq::MessageQ;
			using com::iota::iri::storage::Tangle;
			using org::slf4j::Logger;
			using org::slf4j::LoggerFactory;

			LedgerValidator::LedgerValidator(Tangle *tangle, Milestone *milestone, TransactionRequester *transactionRequester, MessageQ *messageQ) : tangle(tangle), milestone(milestone), transactionRequester(transactionRequester), messageQ(messageQ)
			{
			}

			std::unordered_map<Hash*, long long> LedgerValidator::getLatestDiff(Set<Hash*> *const visitedNonMilestoneSubtangleHashes, Hash *tip, int latestSnapshotIndex, bool milestone) throw(std::exception)
			{
				std::unordered_map<Hash*, long long> state;
				int numberOfAnalyzedTransactions = 0;
				Set<Hash*> *countedTx = std::unordered_set<Hash*>(Collections::singleton(Hash::NULL_HASH));

				visitedNonMilestoneSubtangleHashes->add(Hash::NULL_HASH);

				const std::list<Hash*> nonAnalyzedTransactions = std::list<Hash*>(Collections::singleton(tip));
				Hash *transactionPointer;
				while ((transactionPointer = nonAnalyzedTransactions.pop_front()) != nullptr)
				{
					if (visitedNonMilestoneSubtangleHashes->add(transactionPointer))
					{

						TransactionViewModel * const transactionViewModel = TransactionViewModel::fromHash(tangle, transactionPointer);
						if (transactionViewModel->snapshotIndex() == 0 || transactionViewModel->snapshotIndex() > latestSnapshotIndex)
						{
							numberOfAnalyzedTransactions++;
							if (transactionViewModel->getType() == TransactionViewModel::PREFILLED_SLOT)
							{
								transactionRequester->requestTransaction(transactionViewModel->getHash(), milestone);
								return nullptr;

							}
							else
							{

								if (transactionViewModel->getCurrentIndex() == 0)
								{

									bool validBundle = false;

									const std::vector<std::vector<TransactionViewModel*>> bundleTransactions = BundleValidator::validate(tangle, transactionViewModel->getHash());
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
										if (bundleTransactionViewModels[0]->getHash().equals(transactionViewModel->getHash()))
										{

											validBundle = true;

											for (auto bundleTransactionViewModel : bundleTransactionViewModels)
											{

												if (bundleTransactionViewModel->value() != 0 && countedTx->add(bundleTransactionViewModel->getHash()))
												{

													Hash * const address = bundleTransactionViewModel->getAddressHash();
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

								nonAnalyzedTransactions.push_back(transactionViewModel->getTrunkTransactionHash());
								nonAnalyzedTransactions.push_back(transactionViewModel->getBranchTransactionHash());
							}
						}
					}
				}

				log->debug(L"Analyzed transactions = " + std::to_wstring(numberOfAnalyzedTransactions));
				if (tip == nullptr)
				{
					numberOfConfirmedTransactions = numberOfAnalyzedTransactions;
				}
				log->debug(L"Confirmed transactions = " + std::to_wstring(numberOfConfirmedTransactions));
				return state;
			}

			void LedgerValidator::updateSnapshotMilestone(Hash *hash, int index) throw(std::exception)
			{
				Set<Hash*> *visitedHashes = std::unordered_set<Hash*>();
				const std::list<Hash*> nonAnalyzedTransactions = std::list<Hash*>(Collections::singleton(hash));
				Hash *hashPointer;
				while ((hashPointer = nonAnalyzedTransactions.pop_front()) != nullptr)
				{
					if (visitedHashes->add(hashPointer))
					{
						TransactionViewModel * const transactionViewModel2 = TransactionViewModel::fromHash(tangle, hashPointer);
						if (transactionViewModel2->snapshotIndex() == 0)
						{
							transactionViewModel2->setSnapshot(tangle, index);
							messageQ->publish(L"%s %s %d sn", transactionViewModel2->getAddressHash(), transactionViewModel2->getHash(), index);
							messageQ->publish(L"sn %d %s %s %s %s %s", index, transactionViewModel2->getHash(), transactionViewModel2->getAddressHash(), transactionViewModel2->getTrunkTransactionHash(), transactionViewModel2->getBranchTransactionHash(), transactionViewModel2->getBundleHash());
							nonAnalyzedTransactions.push_back(transactionViewModel2->getTrunkTransactionHash());
							nonAnalyzedTransactions.push_back(transactionViewModel2->getBranchTransactionHash());
						}
					}
				}
			}

			void LedgerValidator::updateConsistentHashes(Set<Hash*> *const visitedHashes, Hash *tip, int index) throw(std::exception)
			{
				const std::list<Hash*> nonAnalyzedTransactions = std::list<Hash*>(Collections::singleton(tip));
				Hash *hashPointer;
				while ((hashPointer = nonAnalyzedTransactions.pop_front()) != nullptr)
				{
					TransactionViewModel * const transactionViewModel2 = TransactionViewModel::fromHash(tangle, hashPointer);
					if ((transactionViewModel2->snapshotIndex() == 0 || transactionViewModel2->snapshotIndex() > index))
					{
						if (visitedHashes->add(hashPointer))
						{
							nonAnalyzedTransactions.push_back(transactionViewModel2->getTrunkTransactionHash());
							nonAnalyzedTransactions.push_back(transactionViewModel2->getBranchTransactionHash());
						}
					}
				}
			}

			void LedgerValidator::init() throw(std::exception)
			{
				MilestoneViewModel *latestConsistentMilestone = buildSnapshot();
				if (latestConsistentMilestone != nullptr)
				{
					log->info(L"Loaded consistent milestone: #" + std::to_wstring(latestConsistentMilestone->index()));

					milestone->latestSolidSubtangleMilestone = latestConsistentMilestone->getHash();
					milestone->latestSolidSubtangleMilestoneIndex = latestConsistentMilestone->index();
				}
			}

			MilestoneViewModel *LedgerValidator::buildSnapshot() throw(std::exception)
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
							StringBuilder *logMessage = new StringBuilder();

							logMessage->append(L"Building snapshot... Consistent: #");
							logMessage->append(consistentMilestone != nullptr ? consistentMilestone->index() : -1);
							logMessage->append(L", Candidate: #");
							logMessage->append(candidateMilestone->index());

							log->info(logMessage->toString());
						}
						if (StateDiffViewModel::maybeExists(tangle, candidateMilestone->getHash()))
						{
							StateDiffViewModel *stateDiffViewModel = StateDiffViewModel::load(tangle, candidateMilestone->getHash());

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

			bool LedgerValidator::updateSnapshot(MilestoneViewModel *milestoneVM) throw(std::exception)
			{
				TransactionViewModel *transactionViewModel = TransactionViewModel::fromHash(tangle, milestoneVM->getHash());
				milestone->latestSnapshot.rwlock.writeLock().lock();
				try
				{
					constexpr int transactionSnapshotIndex = transactionViewModel->snapshotIndex();
					bool hasSnapshot = transactionSnapshotIndex != 0;
					if (!hasSnapshot)
					{
						Hash *tail = transactionViewModel->getHash();
						std::unordered_map<Hash*, long long> currentState = getLatestDiff(std::unordered_set<Hash*, long long>(), tail, milestone->latestSnapshot.index(), true);
						hasSnapshot = currentState.size() > 0 && Snapshot::isConsistent(milestone->latestSnapshot.patchedDiff(currentState));
						if (hasSnapshot)
						{
							updateSnapshotMilestone(milestoneVM->getHash(), milestoneVM->index());
							StateDiffViewModel *stateDiffViewModel;
							stateDiffViewModel = new StateDiffViewModel(currentState, milestoneVM->getHash());
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

			bool LedgerValidator::checkConsistency(std::vector<Hash*> &hashes) throw(std::exception)
			{
				Set<Hash*> *visitedHashes = std::unordered_set<Hash*>();
				std::unordered_map<Hash*, long long> diff;
				for (auto hash : hashes)
				{
					if (!updateDiff(visitedHashes, diff, hash))
					{
						return false;
					}
				}
				return true;
			}

			bool LedgerValidator::updateDiff(Set<Hash*> *approvedHashes, std::unordered_map<Hash*, long long> &diff, Hash *tip) throw(std::exception)
			{
				if (!TransactionViewModel::fromHash(tangle, tip).isSolid())
				{
					return false;
				}
				if (approvedHashes->contains(tip))
				{
					return true;
				}
				Set<Hash*> *visitedHashes = std::unordered_set<Hash*>(approvedHashes);
				std::unordered_map<Hash*, long long> currentState = getLatestDiff(visitedHashes, tip, milestone->latestSnapshot.index(), false);
				if (currentState.empty())
				{
					return false;
				}
				diff.forEach([&] (key, value)
				{
						if (currentState.computeIfPresent(key, ([&] (hash, aLong)
						{
							return value + aLong;
						})) == nullptr)
						{
			currentState.putIfAbsent(key, value);
						}
				});
				bool isConsistent = Snapshot::isConsistent(milestone->latestSnapshot.patchedDiff(currentState));
				if (isConsistent)
				{
					diff.putAll(currentState);
					approvedHashes->addAll(visitedHashes);
				}
				return isConsistent;
			}
		}
	}
}
