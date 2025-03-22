const fetch_from_pagoda = async () => {
  const body = {
    query:
      "query GetLatestSnapshot($offset: Int = 0, $limit: Int = 10, $where: polyprogrammist_near_devhub_prod_v1_proposals_with_latest_snapshot_bool_exp = {}) {\n    polyprogrammist_near_devhub_prod_v1_proposals_with_latest_snapshot(\n      offset: $offset\n      limit: $limit\n      order_by: {proposal_id: desc}\n      where: $where\n    ) {\n      author_id\n      block_height\n      name\n      category\n      summary\n      editor_id\n      proposal_id\n      ts\n      timeline\n      views\n      labels\n      linked_rfp\n    }\n    polyprogrammist_near_devhub_prod_v1_proposals_with_latest_snapshot_aggregate(\n      order_by: {proposal_id: desc}\n      where: $where\n    )  {\n      aggregate {\n        count\n      }\n    }\n  }",
    variables: {
      limit: LIMIT,
      offset: OFFSET,
      where: {},
    },
    operationName: "GetLatestSnapshot",
  };
  const response = await fetch(
    `https://near-queryapi.api.pagoda.co/v1/graphql`,
    {
      method: "POST",
      headers: { "x-hasura-role": "polyprogrammist_near" },
      body: JSON.stringify(body),
    }
  );
  let json = await response.json();
  // console.log(json);
  return json;
};

const fetch_from_cache = async () => {
  const response = await fetch(
    `https://devhub-cache-api-rs-2.fly.dev/proposals?limit=${LIMIT}&offset=${OFFSET}`,
    {
      method: "GET",
      headers: { "Content-Type": "application/json" },
    }
  );
  let json = await response.json();
  return json;
};

const print_all_object_diffs = (cache_obj, pagoda_obj) => {
  let differences = [];

  Object.keys(cache_obj).forEach((key) => {
    if (
      key === "views" ||
      key === "proposal_body_version" ||
      key === "proposal_version"
    ) {
      return;
    }
    if (key === "description") {
      if (SKIP_DESCRIPTION) {
        return;
      }
    }
    if (key === "labels") {
      let cache_labels = cache_obj[key].length;
      let pagoda_labels = pagoda_obj[key].length;
      if (JSON.stringify(cache_labels) !== JSON.stringify(pagoda_labels)) {
        differences.push({
          key,
          cache: cache_labels,
          pagoda: pagoda_labels,
        });
      }
    } else if (key === "timeline") {
      if (SKIP_TIMELINE) {
        return;
      }
      if (cache_obj[key].status !== pagoda_obj[key].status) {
        differences.push({
          key,
          cache: cache_obj[key],
          pagoda: pagoda_obj[key],
          block_height: cache_obj.block_height,
        });
      }
    } else if (cache_obj[key] !== pagoda_obj[key]) {
      differences.push({
        key,
        cache: cache_obj[key],
        pagoda: pagoda_obj[key],
        block_height: cache_obj.block_height,
      });
    }
  });

  if (differences.length > 0) {
    console.log("\nFound differences:");
    differences.forEach((diff) => {
      console.log(`  ${diff.key}:`);
      console.log(`    cache: `, diff.cache);
      console.log(`    pagoda: `, diff.pagoda);
      console.log(` on block: ${diff.block_height}`);
    });
    console.log();
  } else {
    console.log(" No differences found");
  }
};

const compare_results = async () => {
  const cache_result = await fetch_from_cache();
  const pagoda_result = await fetch_from_pagoda();
  let pagoda_records =
    pagoda_result.data
      .polyprogrammist_near_devhub_prod_v1_proposals_with_latest_snapshot;
  let cache_records = cache_result.records;
  let pagoda_total =
    pagoda_result.data
      .polyprogrammist_near_devhub_prod_v1_proposals_with_latest_snapshot_aggregate
      .aggregate;

  let cache_total = cache_result.total_records;

  // console.log(
  //   "Pagoda ids: ",
  //   pagoda_records.map((p) => [p.proposal_id, JSON.parse(p.timeline).status])
  // );
  // console.log(
  //   "Cache ids: ",
  //   cache_records.map((c) => [c.proposal_id, JSON.parse(c.timeline).status])
  // );

  for (let i = 0; i < pagoda_records.length; i++) {
    console.log(`proposal_id: ${pagoda_records[i].proposal_id} snapshot ${i}`);
    print_all_object_diffs(cache_records[i], pagoda_records[i]);
  }

  console.log("pagoda_total, cache_total", pagoda_total, cache_total);
};

const LIMIT = 10;
const OFFSET = 10;
const SKIP_TIMELINE = false;
const SKIP_DESCRIPTION = true;

compare_results();
