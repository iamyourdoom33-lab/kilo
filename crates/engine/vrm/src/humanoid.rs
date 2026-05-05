//! Humanoid bone mapping for VRM models.

//! Maps VRM humanoid bones to standard naming:
//! - hips, spine, chest, neck, head
//! - left/right shoulders, upper arms, lower arms, hands
//! - left/right upper legs, lower legs, feet

pub const HUMANOTD_BONE_NAMES: [&str; 52] = [
    "hips", "spine", "chest", "neck", "head",
    "leftShoulder", "leftUpperArm", "leftLowerArm", "leftHand",
    "rightShoulder", "rightUpperArm", "rightLowerArm", "rightHand",
    "leftUpperLeg", "leftLowerLeg", "leftFoot", "leftToes",
    "rightUpperLeg", "rightLowerLeg", "rightFoot", "rightToes",
    // finger bones...
];
