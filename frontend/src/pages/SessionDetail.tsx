// Copyright 2023 The Matrix.org Foundation C.I.C.
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

import { useAtomValue } from "jotai";

import { currentUserIdAtom } from "../atoms";
import GraphQLError from "../components/GraphQLError";
import NotLoggedIn from "../components/NotLoggedIn";
import UserSessionDetail from "../components/SessionDetail";
import { isErr, unwrapErr, unwrapOk } from "../result";

const SessionDetail: React.FC<{ deviceId: string }> = ({ deviceId }) => {
  const result = useAtomValue(currentUserIdAtom);
  if (isErr(result)) return <GraphQLError error={unwrapErr(result)} />;

  const userId = unwrapOk(result);
  if (userId === null) return <NotLoggedIn />;

  return <UserSessionDetail userId={userId} deviceId={deviceId} />;
};

export default SessionDetail;
