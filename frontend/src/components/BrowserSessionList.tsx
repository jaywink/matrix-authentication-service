// Copyright 2022 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import { atom, useAtom, useAtomValue, useSetAtom } from "jotai";
import { atomFamily } from "jotai/utils";
import { atomWithQuery } from "jotai-urql";
import { useTransition } from "react";

import { mapQueryAtom } from "../atoms";
import { graphql } from "../gql";
import { SessionState, PageInfo } from "../gql/graphql";
import {
  atomForCurrentPagination,
  atomWithPagination,
  FIRST_PAGE,
  Pagination,
} from "../pagination";
import { isOk, unwrap, unwrapOk } from "../result";

import BlockList from "./BlockList";
import BrowserSession from "./BrowserSession";
import PaginationControls from "./PaginationControls";
import SessionListHeader from "./SessionList/SessionListHeader";

const QUERY = graphql(/* GraphQL */ `
  query BrowserSessionList(
    $userId: ID!
    $state: SessionState
    $first: Int
    $after: String
    $last: Int
    $before: String
  ) {
    user(id: $userId) {
      id
      browserSessions(
        first: $first
        after: $after
        last: $last
        before: $before
        state: $state
      ) {
        totalCount

        edges {
          cursor
          node {
            id
            ...BrowserSession_session
          }
        }

        pageInfo {
          hasNextPage
          hasPreviousPage
          startCursor
          endCursor
        }
      }
    }
  }
`);

const filterAtom = atom<SessionState | null>(SessionState.Active);
const currentPaginationAtom = atomForCurrentPagination();

const browserSessionListFamily = atomFamily((userId: string) => {
  const browserSessionListQuery = atomWithQuery({
    query: QUERY,
    getVariables: (get) => ({
      userId,
      state: get(filterAtom),
      ...get(currentPaginationAtom),
    }),
  });

  const browserSessionList = mapQueryAtom(
    browserSessionListQuery,
    (data) => data.user?.browserSessions || null,
  );

  return browserSessionList;
});

const pageInfoFamily = atomFamily((userId: string) => {
  const pageInfoAtom = atom(async (get): Promise<PageInfo | null> => {
    const result = await get(browserSessionListFamily(userId));
    return (isOk(result) && unwrapOk(result)?.pageInfo) || null;
  });
  return pageInfoAtom;
});

const paginationFamily = atomFamily((userId: string) => {
  const paginationAtom = atomWithPagination(
    currentPaginationAtom,
    pageInfoFamily(userId),
  );

  return paginationAtom;
});

const BrowserSessionList: React.FC<{ userId: string }> = ({ userId }) => {
  const [pending, startTransition] = useTransition();
  const result = useAtomValue(browserSessionListFamily(userId));
  const setPagination = useSetAtom(currentPaginationAtom);
  const [prevPage, nextPage] = useAtomValue(paginationFamily(userId));
  const [filter, setFilter] = useAtom(filterAtom);

  const browserSessions = unwrap(result);
  if (browserSessions === null) return <>Failed to load browser sessions</>;

  const paginate = (pagination: Pagination): void => {
    startTransition(() => {
      setPagination(pagination);
    });
  };

  const toggleFilter = (): void => {
    startTransition(() => {
      setPagination(FIRST_PAGE);
      setFilter(filter === SessionState.Active ? null : SessionState.Active);
    });
  };

  return (
    <BlockList>
      <SessionListHeader title="Browsers" />
      <PaginationControls
        onPrev={prevPage ? (): void => paginate(prevPage) : null}
        onNext={nextPage ? (): void => paginate(nextPage) : null}
        count={browserSessions.totalCount}
        disabled={pending}
      />
      <label>
        <input
          type="checkbox"
          checked={filter === SessionState.Active}
          onChange={toggleFilter}
        />{" "}
        Active only
      </label>
      {browserSessions.edges.map((n) => (
        <BrowserSession key={n.cursor} session={n.node} />
      ))}
    </BlockList>
  );
};

export default BrowserSessionList;
