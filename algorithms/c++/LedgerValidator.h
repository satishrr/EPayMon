#pragma once

//========================================================================
// This conversion was produced by the Free Edition of
// Java to C++ Converter courtesy of Tangible Software Solutions.
// Order the Premium Edition at https://www.tangiblesoftwaresolutions.com
//========================================================================

#include <unordered_map>
#include <vector>
#include <list>
#include <stdexcept>
#include <boost/optional.hpp>
#include "stringbuilder.h"

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


			/// <summary>
			/// Created by paul on 4/15/17.
			/// </summary>
			class LedgerValidator
			{

			private:
				Logger *const log = LoggerFactory::getLogger(LedgerValidator::typeid);
				Tangle *const tangle;
				Milestone *const milestone;
				TransactionRequester *const transactionRequester;
				MessageQ *const messageQ;
//JAVA TO C++ CONVERTER TODO TASK: 'volatile' has a different meaning in C++:
//ORIGINAL LINE: private volatile int numberOfConfirmedTransactions;
				int numberOfConfirmedTransactions = 0;

			public:
				virtual ~LedgerValidator()
				{
					delete log;
					delete tangle;
					delete milestone;
					delete transactionRequester;
					delete messageQ;
				}

				LedgerValidator(Tangle *tangle, Milestone *milestone, TransactionRequester *transactionRequester, MessageQ *messageQ);

				/// <summary>
				/// Returns a Map of Address and change in balance that can be used to build a new Snapshot state.
				/// Under certain conditions, it will return null:
				///  - While descending through transactions, if a transaction is marked as {PREFILLED_SLOT}, then its hash has been
				///    referenced by some transaction, but the transaction data is not found in the database. It notifies
				///    TransactionRequester to increase the probability this transaction will be present the next time this is checked.
				///  - When a transaction marked as a tail transaction (if the current index is 0), but it is not the first transaction
				///    in any of the BundleValidator's transaction lists, then the bundle is marked as invalid, deleted, and re-requested.
				///  - When the bundle is not internally consistent (the sum of all transactions in the bundle must be zero)
				/// As transactions are being traversed, it will come upon bundles, and will add the transaction value to {state}.
				/// If {milestone} is true, it will search, through trunk and branch, all transactions, starting from {tip},
				/// until it reaches a transaction that is marked as a "confirmed" transaction.
				/// If {milestone} is false, it will search up until it reaches a confirmed transaction, or until it finds a hash that has been
				/// marked as consistent since the previous milestone. </summary>
				/// <param name="visitedNonMilestoneSubtangleHashes"> hashes that have been visited and considered as approved </param>
				/// <param name="tip">                                the hash of a transaction to start the search from </param>
				/// <param name="latestSnapshotIndex">                index of the latest snapshot to traverse to </param>
				/// <param name="milestone">                          marker to indicate whether to stop only at confirmed transactions </param>
				/// <returns> {state}                           the addresses that have a balance changed since the last diff check </returns>
				/// <exception cref="Exception"> </exception>
				virtual std::unordered_map<Hash*, long long> getLatestDiff(Set<Hash*> *const visitedNonMilestoneSubtangleHashes, Hash *tip, int latestSnapshotIndex, bool milestone) throw(std::exception);

				/// <summary>
				/// Descends through the tree of transactions, through trunk and branch, marking each as {mark} until it reaches
				/// a transaction while the transaction confirmed marker is mutually exclusive to {mark} </summary>
				/// // old <param name="hash"> start of the update tree </param>
				/// <param name="hash"> tail to traverse from </param>
				/// <param name="index"> milestone index </param>
				/// <exception cref="Exception"> </exception>
			private:
				void updateSnapshotMilestone(Hash *hash, int index) throw(std::exception);

				/// <summary>
				/// Descends through transactions, trunk and branch, beginning at {tip}, until it reaches a transaction marked as
				/// confirmed, or until it reaches a transaction that has already been added to the transient consistent set. </summary>
				/// <param name="tip"> </param>
				/// <exception cref="Exception"> </exception>
				void updateConsistentHashes(Set<Hash*> *const visitedHashes, Hash *tip, int index) throw(std::exception);

				/// <summary>
				/// Initializes the LedgerValidator. This updates the latest milestone and solid subtangle milestone, and then
				/// builds up the confirmed until it reaches the latest consistent confirmed. If any inconsistencies are detected,
				/// perhaps by database corruption, it will delete the milestone confirmed and all that follow.
				/// It then starts at the earliest consistent milestone index with a confirmed, and analyzes the tangle until it
				/// either reaches the latest solid subtangle milestone, or until it reaches an inconsistent milestone. </summary>
				/// <exception cref="Exception"> </exception>
			protected:
				virtual void init() throw(std::exception);

				/// <summary>
				/// Only called once upon initialization, this builds the {latestSnapshot} state up to the most recent
				/// solid milestone confirmed. It gets the earliest confirmed, and while checking for consistency, patches the next
				/// newest confirmed diff into its map. </summary>
				/// <returns>              the most recent consistent milestone with a confirmed. </returns>
				/// <exception cref="Exception"> </exception>
			private:
				MilestoneViewModel *buildSnapshot() throw(std::exception);

			public:
				virtual bool updateSnapshot(MilestoneViewModel *milestoneVM) throw(std::exception);

				virtual bool checkConsistency(std::vector<Hash*> &hashes) throw(std::exception);

				virtual bool updateDiff(Set<Hash*> *approvedHashes, std::unordered_map<Hash*, long long> &diff, Hash *tip) throw(std::exception);
			};
		}
	}
}
