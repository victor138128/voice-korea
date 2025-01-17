use dioxus_translate::translate;

translate! {
    OpinionNewTranslate;

    organization_management: {
        ko: "조직 관리",
        en: "Organization Management",
    },
    public_opinion_management: {
        ko: "공론관리",
        en: "Public Opinion Management"
    },
    start_public_opinion: {
        ko: "공론 시작하기",
        en: "Start Public Opinion"
    }
    organization_and_period_title: {
        ko: "공론 절차 구성 및 기간",
        en: "Organization and period of public deliberation procedures"
    }

    duration_per_procedure: {
        ko: "절차별 기간",
        en: "Duration per procedure"
    }
    duration_per_procedure_description: {
        ko: "공론의 방식과 순서를 정해주세요.",
        en: "Please decide the method and order of public discussion."
    }
    no_contents: {
        ko: "내용 없음",
        en: "No Contents"
    }
    required_period_selection: {
        ko: "기간 선택 필요",
        en: "Period selection required"
    }
    starting_day: {
        ko: "시작하는 날",
        en: "Starting day"
    }
    last_day: {
        ko: "마지막 날",
        en: "Last day"
    }

    organization_of_procedures: {
        ko: "공론 절차 구성",
        en: "Organization of public opinion procedures"
    }
    organization_of_procedures_description: {
        ko: "공론의 방식과 순서를 정해주세요.",
        en: "Please decide the method and order of public discussion."
    }
    no_selection: {
        ko: "선택 없음",
        en: "No Selection"
    }
    remove: {
        ko: "삭제",
        en: "Remove"
    }
    to_public_opinion_management_list: {
        ko: "공론관리 목록으로",
        en: "To public opinion management list"
    }
    temporary_save: {
        ko: "임시저장",
        en: "Temporary Save"
    }
    next: {
        ko: "다음으로",
        en: "Next"
    }

    regular_post: {
        ko: "일반 게시글",
        en: "Regular Post"
    }
    video_conference: {
        ko: "화상 회의",
        en: "Video Conference"
    }
    post: {
        ko: "포스트형 게시글",
        en: "Post"
    }
    vote: {
        ko: "투표",
        en: "Vote"
    }
    report: {
        ko: "보고서",
        en: "Report"
    }

    information_provided: {
        ko: "정보 제공",
        en: "Information Provided"
    }
    discussion_and_deliberation: {
        ko: "토론 및 숙의",
        en: "Discussion And Deliberation"
    }
    derive_opinions: {
        ko: "의견 도출",
        en: "Derive Opinions"
    }
    reach_consensus: {
        ko: "합의 도출",
        en: "Reach Consensus"
    }
    analysis_result: {
        ko: "결과 분석",
        en: "Analysis Result"
    }
}

translate! {
    InputIntroductionTranslate;

    enter_introduction: {
        ko: "소개글 입력",
        en: "Enter Introduction"
    }
    introduction_description: {
        ko: "공론의 주제와 목적에 대해 설명해주세요. 참여자들이 더 쉽게 이해하고 적극적으로 참여할 수 있을 것입니다.",
        en: "Please explain the topic and purpose of the public debate. Participants will be able to understand more easily and participate actively."
    }
    select_field: {
        ko: "분야 선택",
        en: "Select Field"
    }
    enter_title_hint: {
        ko: "제목을 입력해주세요.",
        en: "Please enter a title."
    }
    enter_description_hint: {
        ko: "내용을 입력해주세요.",
        en: "Please enter a description."
    }
}

translate! {
    InputOpinionTranslate;

    essential_information: {
        ko: "공론 필수 정보",
        en: "Public opinion essential information"
    }
    backward: {
        ko: "뒤로",
        en: "Backward"
    }
    temporary_save: {
        ko: "임시저장",
        en: "Temporary Save"
    }
    next: {
        ko: "다음으로",
        en: "Next"
    }
}

translate! {
    UploadDocumentTranslate;

    upload_document: {
        ko: "자료 업로드",
        en: "Upload Document"
    }
    upload_document_description: {
        ko: "해당 공론과 관련된 자료를 업로드해주세요. (예. 공론 소개, 참여 방법, 가이드라인)",
        en: "Please upload materials related to the public opinion. (e.g. introduction to public opinion, participation methods, guidelines)"
    }
    direct_upload: {
        ko: "직접 업로드하기",
        en: "Direct Upload"
    }
    import_from_data_management: {
        ko: "자료관리에서 불러오기",
        en: "Import from data management"
    }
    upload_file_description: {
        ko: "업로드할 파일을 드래그해주세요.",
        en: "Please drag the file you want to upload."
    }
    load_file: {
        ko: "파일 불러오기",
        en: "Load File"
    }
}

translate! {
    ImportDocumentTranslate;

    upload_file_warning: {
        ko: "jpg, .png, pdf, zip, word, excel, pptx 파일만 업로드 가능합니다.",
        en: "Only jpg, .png, pdf, zip, word, excel, and pptx files can be uploaded."
    }
    title: {
        ko: "제목",
        en: "Title"
    }
    document_type: {
        ko: "유형",
        en: "Type"
    }
    field: {
        ko: "분야",
        en: "Field"
    }
    purpose_of_use: {
        ko: "활용 목적",
        en: "Purpose of use"
    }
    source: {
        ko: "출처",
        en: "Source"
    }
    authority: {
        ko: "권한",
        en: "Authority"
    }
    last_modified_date: {
        ko: "최종 수정일",
        en: "Last Modified Date"
    }
    form: {
        ko: "형식",
        en: "Form"
    }
}

translate! {
    DirectUploadTranslate;

    upload_file_description: {
        ko: "업로드할 파일을 드래그해주세요.",
        en: "Please drag the file you want to upload."
    }
    load_file: {
        ko: "파일 불러오기",
        en: "Load File"
    }
    upload_file_warning: {
        ko: "jpg, .png, pdf, zip, word, excel, pptx 파일만 업로드 가능합니다.",
        en: "Only jpg, .png, pdf, zip, word, excel, and pptx files can be uploaded."
    }
}

translate! {
    ConnectProjectTranslate;

    research_project_linkage: {
        ko: "조사 프로젝트 연동",
        en: "Research project linkage"
    }
    research_project_linkage_description: {
        ko: "해당 공론과 관련된 조사자료를 불러와주세요. (예. 여론조사, 설문조사, 기타조사 등)",
        en: "Please retrieve research data related to this public opinion. (e.g. opinion polls, surveys, other surveys, etc.)"
    }
    research_selection: {
        ko: "조사 선택",
        en: "Research Selection"
    }
}

translate! {
    CompositionCommitteeTranslate;
    composition_committee_title: {
        ko: "공론 위원회 구성",
        en: "Composition of a public opinion committee"
    }
    composition_committee_description: {
        ko: "공론위원회는 다양한 의견을 수렴하고 합의된 결정을 도출하는 역할을 합니다. 각 역할의 담당자를 설정해주세요.",
        en: "The Public Opinion Committee's role is to collect diverse opinions and arrive at a consensus decision. Please set a person in charge of each role."
    }
    opinion_designer_label: {
        ko: "공론 설계자",
        en: "Public Opinion Designer"
    }
    opinion_designer_hint: {
        ko: "공론 설계자 선택",
        en: "Select Public Opinion Designer"
    }
    specific_opinion_designer_label: {
        ko: "특정 공론 설계자",
        en: "Specific Public Opinion Designer"
    }
    specific_opinion_designer_hint: {
        ko: "특정 공론 설계자 선택",
        en: "Select Specific Public Opinion Designer"
    }
    analyst_label: {
        ko: "분석가",
        en: "Analyst"
    }
    analyst_hint: {
        ko: "분석가 선택",
        en: "Select Analyst"
    }
    intermediary_label: {
        ko: "중개자",
        en: "Intermediary"
    }
    intermediary_hint: {
        ko: "중개자 선택",
        en: "Select Intermediary"
    }
    lecturer_label: {
        ko: "강연자",
        en: "Lecturer"
    }
    lecturer_hint: {
        ko: "강연자 선택",
        en: "Select Lecturer"
    }
    division_roles: {
        ko: "역할 분담",
        en: "Division of Roles"
    }
    backward: {
        ko: "뒤로",
        en: "Backward"
    }
    temporary_save: {
        ko: "임시저장",
        en: "Temporary Save"
    }
    next: {
        ko: "다음으로",
        en: "Next"
    }
}

translate! {
    CompositionPanelTranslate;

    faired_people_allocated: {
        ko: "공평한 인원수 배정",
        en: "Fair number of people allocated"
    }
    participant_panel_composition: {
        ko: "참여자 패널 구성",
        en: "Participant Panel Composition"
    }
    backward: {
        ko: "뒤로",
        en: "Backward"
    }
    temporary_save: {
        ko: "임시저장",
        en: "Temporary Save"
    }
    next: {
        ko: "다음으로",
        en: "Next"
    }
}

translate! {
    DirectedAddPanelTranslate;

    directed_add_panel_title: {
        ko: "패널 직접 추가",
        en: "Add panels directly"
    }
    directed_add_panel_description: {
        ko: "패널별 속성을 직접 작성하여 추가합니다. 이는 패널&속성관리 페이지에 자동으로 반영됩니다.",
        en: "Create and add panel-specific properties yourself. This is automatically reflected on the panel & property management page."
    }
    enter_panel_name: {
        ko: "패널명 입력",
        en: "Enter Panel Name"
    }
    select_attribute: {
        ko: "속성 선택",
        en: "Select Attribute"
    }
}

translate! {
    SettingTotalPanelTranslate;

    setting_total_panel_title: {
        ko: "전체 패널 설정",
        en: "Total Panel Settings"
    }
    setting_total_panel_description: {
        ko: "공론위원회는 다양한 의견을 수렴하고 합의된 결정을 도출하는 역할을 합니다. 각 역할의 담당자를 선정해주세요.",
        en: "The Public Opinion Committee's role is to collect diverse opinions and arrive at a consensus decision. Please select a person in charge of each role."
    }
    total_panel: {
        ko: "전체 패널",
        en: "Total Panel"
    }
    faired_people_allocated: {
        ko: "공평한 인원수 배정",
        en: "Fair number of people allocated"
    }
    proportional_people_allocated: {
        ko: "인원수 비례 배정",
        en: "Proportional allocation of number of people"
    }
    total_members: {
        ko: "총 인원",
        en: "Total Members"
    }
    select_panel: {
        ko: "패널 선택",
        en: "Select Panel"
    }
}
