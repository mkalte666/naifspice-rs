# Implementaion overview 

This list is directly pulled from the spice documentation: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/

"mostused" means that api is listed here: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/info/mostused.html

### A

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `APPNDC_C` | :x: | :x: | _ | _ | Append an item to a character cell
| `APPNDD_C` | :x: | :x: | _ | _ | Append an item to a double precision cell
| `APPNDI_C` | :x: | :x: | _ | _ | Append an item to an integer cell
| `AXISAR_C` | :x: | :heavy_check_mark: | _ | _ | Axis and angle to rotation
| `AZLCPO_C` | :x: | :heavy_check_mark: | _ | _ | AZ/EL, constant position observer state
| `AZLREC_C` | :x: | :heavy_check_mark: | _ | _ | AZ/EL to rectangular coordinates

### B

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `B1900_C` | :x: | :heavy_check_mark: | _ | _ | Besselian Date 1900.0
| `B1950_C` | :x: | :heavy_check_mark: | _ | _ | Besselian Date 1950.0
| `BADKPV_C` | :x: | :x: | _ | _ | Bad Kernel Pool Variable
| `BLTFRM_C` | :x: | :x: | _ | _ | Built-in frame IDs
| `BODC2N_C` | :x: | :heavy_check_mark: | _ | _ | Body ID code to name translation
| `BODC2S_C` | :x: | :x: | _ | _ | Body ID code to string translation
| `BODDEF_C` | :x: | :x: | _ | _ | Body name/ID code definition
| `BODFND_C` | :x: | :heavy_check_mark: | _ | _ | Find values from the kernel pool
| `BODN2C_C` | :x: | :heavy_check_mark: | _ | _ | Body name to ID code translation
| `BODS2C_C` | :x: | :x: | _ | _ | Body string to ID code translation
| `BODVAR_C` | :x: | :x: | _ | _ | Return values from the kernel pool
| `BODVCD_C` | :x: | :x: | _ | _ | Return d.p. values from the kernel pool
| `BODVRD_C` | :x: | :heavy_check_mark: | _ | _ | Return d.p. values from the kernel pool
| `BRCKTD_C` | :x: | :x: | _ | _ | Bracket a d.p. value within an interval
| `BRCKTI_C` | :x: | :x: | _ | _ | Bracket an integer value within an interval
| `BSCHOC_C` | :x: | :x: | _ | _ | Binary search with order vector, character
| `BSCHOI_C` | :x: | :x: | _ | _ | Binary search with order vector, integer
| `BSRCHC_C` | :x: | :x: | _ | _ | Binary search for a character string
| `BSRCHD_C` | :x: | :x: | _ | _ | Binary search for a double precision value
| `BSRCHI_C` | :x: | :x: | _ | _ | Binary search for an integer value

### C

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `CARD_C` | :heavy_check_mark: | :x: | SpiceCell::length | _ | Cardinality of a cell
| `CCIFRM_C` | :x: | :x: | _ | _ | Class and class ID to associated frame
| `CGV2EL_C` | :x: | :x: | _ | _ | Center and generating vectors to ellipse
| `CHBDER_C` | :x: | :x: | _ | _ | Derivatives of a Chebyshev expansion
| `CHBIGR_C` | :x: | :x: | _ | _ | Chebyshev expansion integral
| `CHBINT_C` | :x: | :x: | _ | _ | Interpolate a Chebyshev expansion
| `CHBVAL_C` | :x: | :x: | _ | _ | Value of a Chebyshev polynomial expansion
| `CHKIN_C` | :x: | :x: | _ | _ | module Check In
| `CHKOUT_C` | :x: | :x: | _ | _ | Module Check Out
| `CIDFRM_C` | :x: | :heavy_check_mark: | _ | _ | center SPK ID frame
| `CKCLS_C` | :x: | :x: | _ | _ | CK, Close file
| `CKCOV_C` | :x: | :x: | _ | _ | CK coverage
| `CKFROT_C` | :x: | :x: | _ | _ | CK frame, find position rotation
| `CKFXFM_C` | :x: | :x: | _ | _ | CK frame, find state transformation
| `CKGP_C` | :x: | :x: | _ | _ | C-kernel, get pointing
| `CKGPAV_C` | :x: | :heavy_check_mark: | _ | _ | C-kernel, get pointing and angular velocity
| `CKGR02_C` | :x: | :x: | _ | _ | C-kernel, get record, type 02
| `CKGR03_C` | :x: | :x: | _ | _ | C-kernel, get record, type 03
| `CKLPF_C` | :x: | :x: | _ | _ | CK, load pointing file
| `CKMETA_C` | :x: | :x: | _ | _ | CK ID to associated SCLK
| `CKNR02_C` | :x: | :x: | _ | _ | C-kernel, number of records, type 02
| `CKNR03_C` | :x: | :x: | _ | _ | C-kernel, number of records, type 03
| `CKOBJ_C` | :x: | :heavy_check_mark: | _ | _ | CK objects
| `CKOPN_C` | :x: | :x: | _ | _ | CK, open new file.
| `CKUPF_C` | :x: | :x: | _ | _ | CK, Unload pointing file
| `CKW01_C` | :x: | :x: | _ | _ | C-Kernel, write segment to C-kernel, data type 1
| `CKW02_C` | :x: | :x: | _ | _ | C-Kernel, write segment to C-kernel, data type 2
| `CKW03_C` | :x: | :x: | _ | _ | C-Kernel, write segment to C-kernel, data type 3
| `CKW05_C` | :x: | :x: | _ | _ | Write CK segment, type 5
| `CLEARC_C` | :x: | :x: | _ | _ | Clear a two-dimensional character array
| `CLEARD_C` | :x: | :x: | _ | _ | Clear a double precision array
| `CLEARI_C` | :x: | :x: | _ | _ | Clear an integer array
| `CLIGHT_C` | :x: | :heavy_check_mark: | _ | _ | C, Speed of light in a vacuum
| `CLPOOL_C` | :x: | :x: | _ | _ | Clear the pool of kernel variables
| `CMPRSS_C` | :x: | :x: | _ | _ | Compress a character string
| `CNMFRM_C` | :x: | :heavy_check_mark: | _ | _ | Center name to associated frame
| `CONICS_C` | :x: | :heavy_check_mark: | _ | _ | Determine state from conic elements
| `CONVRT_C` | :x: | :x: | _ | _ | Convert Units
| `COPY_C` | :heavy_minus_sign: | :x: | _ | Handled via derive(clone) | Copy a SPICE cell
| `CPOS_C` | :x: | :x: | _ | _ | Character position
| `CPOSR_C` | :x: | :x: | _ | _ | Character position, reverse
| `CVPOOL_C` | :x: | :x: | _ | _ | Check variable in the pool for update
| `CYLLAT_C` | :x: | :heavy_check_mark: | _ | _ | Cylindrical to latitudinal
| `CYLREC_C` | :x: | :heavy_check_mark: | _ | _ | Cylindrical to rectangular
| `CYLSPH_C` | :x: | :heavy_check_mark: | _ | _ | Cylindrical to spherical

### D

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `DAFAC_C` | :x: | :x: | _ | _ | DAF add comments
| `DAFBBS_C` | :x: | :x: | _ | _ | DAF, begin backward search
| `DAFBFS_C` | :x: | :x: | _ | _ | DAF, begin forward search
| `DAFCLS_C` | :x: | :x: | _ | _ | DAF, close
| `DAFCS_C` | :x: | :x: | _ | _ | DAF, continue search
| `DAFDC_C` | :x: | :x: | _ | _ | DAF delete comments
| `DAFEC_C` | :x: | :x: | _ | _ | DAF extract comments
| `DAFFNA_C` | :x: | :x: | _ | _ | DAF, find next array
| `DAFFPA_C` | :x: | :x: | _ | _ | DAF, find previous array
| `DAFGDA_C` | :x: | :x: | _ | _ | DAF, read data from address
| `DAFGH_C` | :x: | :x: | _ | _ | DAF, get handle
| `DAFGN_C` | :x: | :x: | _ | _ | DAF, get array name
| `DAFGS_C` | :x: | :x: | _ | _ | DAF, get summary
| `DAFGSR_C` | :x: | :x: | _ | _ | DAF, get summary/descriptor record
| `DAFHSF_C` | :x: | :x: | _ | _ | DAF, handle to summary format
| `DAFOPR_C` | :x: | :x: | _ | _ | DAF, open for read
| `DAFOPW_C` | :x: | :x: | _ | _ | DAF, open for write
| `DAFPS_C` | :x: | :x: | _ | _ | DAF, pack summary
| `DAFRDA_C` | :x: | :x: | _ | _ | DAF, read data from address
| `DAFRFR_C` | :x: | :x: | _ | _ | DAF, read file record
| `DAFRS_C` | :x: | :x: | _ | _ | DAF, replace summary
| `DAFUS_C` | :x: | :x: | _ | _ | DAF, unpack summary
| `DASAC_C` | :x: | :x: | _ | _ | DAS add comments
| `DASADC_C` | :x: | :x: | _ | _ | DAS, add data, character
| `DASADD_C` | :x: | :x: | _ | _ | DAS, add data, double precision
| `DASADI_C` | :x: | :x: | _ | _ | DAS, add data, integer
| `DASCLS_C` | :x: | :x: | _ | _ | DAS, close file
| `DASDC_C` | :x: | :x: | _ | _ | DAS, delete comments
| `DASEC_C` | :x: | :x: | _ | _ | DAS extract comments
| `DASHFN_C` | :x: | :x: | _ | _ | DAS, handle to file name
| `DASHFS_C` | :x: | :x: | _ | _ | DAS, handle to file summary
| `DASLLA_C` | :x: | :x: | _ | _ | DAS, last logical addresses
| `DASLLC_C` | :x: | :x: | _ | _ | DAS, low-level close
| `DASONW_C` | :x: | :x: | _ | _ | DAS, open new file
| `DASOPR_C` | :x: | :x: | _ | _ | DAS, open for read
| `DASOPS_C` | :x: | :x: | _ | _ | DAS, open scratch
| `DASOPW_C` | :x: | :x: | _ | _ | DAS, open for write
| `DASRDC_C` | :x: | :x: | _ | _ | DAS, read data, character
| `DASRDD_C` | :x: | :x: | _ | _ | DAS, read data, double precision
| `DASRDI_C` | :x: | :x: | _ | _ | DAS, read data, integer
| `DASRFR_C` | :x: | :x: | _ | _ | DAS, read file record
| `DASUDC_C` | :x: | :x: | _ | _ | DAS, update data, character
| `DASUDD_C` | :x: | :x: | _ | _ | DAS, update data, double precision
| `DASUDI_C` | :x: | :x: | _ | _ | DAS, update data, integer
| `DASWBR_C` | :x: | :x: | _ | _ | DAS, write buffered records
| `DAZLDR_C` | :x: | :x: | _ | _ | Derivative of AZ/EL w.r.t. rectangular
| `DCYLDR_C` | :x: | :x: | _ | _ | Derivative of cylindrical w.r.t. rectangular
| `DELTET_C` | :x: | :x: | _ | _ | Delta ET, ET | :x: | :x: | _ | _ | UTC
| `DET_C` | :x: | :heavy_check_mark: | _ | _ | Determinant of a double precision 3x3 matrix
| `DGEODR_C` | :x: | :x: | _ | _ | Derivative of geodetic w.r.t. rectangular
| `DIAGS2_C` | :x: | :x: | _ | _ | Diagonalize symmetric 2x2 matrix
| `DIFF_C` | :x: | :x: | _ | _ | Difference of two sets
| `DLABBS_C` | :x: | :x: | _ | _ | DLA, begin backward search
| `DLABFS_C` | :x: | :x: | _ | _ | DLA, begin forward search
| `DLABNS_C` | :x: | :x: | _ | _ | DLA, begin new segment
| `DLAENS_C` | :x: | :x: | _ | _ | DLA, end new segment
| `DLAFNS_C` | :x: | :x: | _ | _ | DLA, find next segment
| `DLAFPS_C` | :x: | :x: | _ | _ | DLA, find previous segment
| `DLAOPN_C` | :x: | :x: | _ | _ | DLA, open new file
| `DLATDR_C` | :x: | :x: | _ | _ | Derivative of latitudinal w.r.t. rectangular
| `DNEARP_C` | :x: | :x: | _ | _ | Derivative of near point
| `DP2HX_C` | :x: | :x: | _ | _ | D.p. number to hexadecimal string
| `DPGRDR_C` | :x: | :x: | _ | _ | Derivative of planetographic w.r.t. rectangular
| `DPMAX_C` | :x: | :x: | _ | _ | Largest DP number
| `DPMIN_C` | :x: | :x: | _ | _ | Smallest DP number
| `DPR_C` | :x: | :heavy_check_mark: | _ | _ | Degrees per radian
| `DRDAZL_C` | :x: | :x: | _ | _ | Derivative of rectangular w.r.t. AZ/EL
| `DRDCYL_C` | :x: | :x: | _ | _ | Derivative of rectangular w.r.t. cylindrical
| `DRDGEO_C` | :x: | :x: | _ | _ | Derivative of rectangular w.r.t. geodetic
| `DRDLAT_C` | :x: | :x: | _ | _ | Derivative of rectangular w.r.t. latitudinal
| `DRDPGR_C` | :x: | :x: | _ | _ | Derivative of rectangular w.r.t. planetographic
| `DRDSPH_C` | :x: | :x: | _ | _ | Derivative of rectangular w.r.t. spherical
| `DSKB02_C` | :x: | :x: | _ | _ | DSK, fetch type 2 bookkeeping data
| `DSKCLS_C` | :x: | :x: | _ | _ | DSK, close file
| `DSKD02_C` | :x: | :x: | _ | _ | DSK, fetch d.p. type 2 data
| `DSKGD_C` | :x: | :x: | _ | _ | DSK, return DSK segment descriptor
| `DSKGTL_C` | :x: | :x: | _ | _ | DSK, get tolerance
| `DSKI02_C` | :x: | :x: | _ | _ | DSK, fetch integer type 2 data
| `DSKMI2_C` | :x: | :x: | _ | _ | DSK, make spatial index for type 2 segment
| `DSKN02_C` | :x: | :x: | _ | _ | DSK, type 2, compute normal vector for plate
| `DSKOBJ_C` | :x: | :heavy_check_mark: | _ | _ | DSK, get object IDs
| `DSKOPN_C` | :x: | :x: | _ | _ | DSK, open new file
| `DSKP02_C` | :x: | :heavy_check_mark: | _ | _ | DSK, fetch type 2 plate data
| `DSKRB2_C` | :x: | :x: | _ | _ | DSK, determine range bounds for plate set
| `DSKSRF_C` | :x: | :heavy_check_mark: | _ | _ | DSK, get surface IDs for body
| `DSKSTL_C` | :x: | :x: | _ | _ | DSK, set tolerance
| `DSKV02_C` | :x: | :x: | _ | _ | DSK, fetch type 2 vertex data
| `DSKW02_C` | :x: | :x: | _ | _ | DSK, write type 2 segment
| `DSKX02_C` | :x: | :x: | _ | _ | DSK, ray-surface intercept, type 2
| `DSKXSI_C` | :x: | :heavy_check_mark: | _ | _ | DSK, ray-surface intercept with source information
| `DSKXV_C` | :x: | :heavy_check_mark: | _ | _ | DSK, ray-surface intercept, vectorized
| `DSKZ02_C` | :x: | :heavy_check_mark: | _ | _ | DSK, fetch type 2 model size parameters
| `DSPHDR_C` | :x: | :x: | _ | _ | Derivative of spherical w.r.t. rectangular
| `DTPOOL_C` | :x: | :x: | _ | _ | Data for a kernel pool variable
| `DUCRSS_C` | :x: | :x: | _ | _ | Unit Normalized Cross Product and Derivative
| `DVCRSS_C` | :x: | :heavy_check_mark: | _ | _ | Derivative of Vector cross product
| `DVDOT_C` | :x: | :heavy_check_mark: | _ | _ | Derivative of Vector Dot Product, 3-D
| `DVHAT_C` | :x: | :heavy_check_mark: | _ | _ | Derivative and unit vector "V-hat" of a state
| `DVNORM_C` | :x: | :x: | _ | _ | Derivative of vector norm
| `DVPOOL_C` | :x: | :x: | _ | _ | Delete a variable from the kernel pool
| `DVSEP_C` | :x: | :x: | _ | _ | Time derivative of separation angle

### E

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `EDLIMB_C` | :x: | :x: | _ | _ | Ellipsoid Limb
| `EDNMPT_C` | :x: | :x: | _ | _ | Ellipsoid normal vector to surface point
| `EDPNT_C` | :x: | :x: | _ | _ | Ellipsoid point
| `EDTERM_C` | :x: | :x: | _ | _ | Ellipsoid terminator
| `EKACEC_C` | :x: | :x: | _ | _ | EK, add character data to column
| `EKACED_C` | :x: | :x: | _ | _ | EK, add d.p. data to column
| `EKACEI_C` | :x: | :x: | _ | _ | EK, add integer data to column
| `EKACLC_C` | :x: | :x: | _ | _ | EK, add character column to segment
| `EKACLD_C` | :x: | :x: | _ | _ | EK, add double precision column to segment
| `EKACLI_C` | :x: | :x: | _ | _ | EK, add integer column to segment
| `EKAPPR_C` | :x: | :x: | _ | _ | EK, append record onto segment
| `EKBSEG_C` | :x: | :x: | _ | _ | EK, start new segment
| `EKCCNT_C` | :x: | :x: | _ | _ | EK, column count
| `EKCII_C` | :x: | :x: | _ | _ | EK, column info by index
| `EKCLS_C` | :x: | :x: | _ | _ | EK, close file
| `EKDELR_C` | :x: | :x: | _ | _ | EK, delete record from segment
| `EKFFLD_C` | :x: | :x: | _ | _ | EK, finish fast write
| `EKFIND_C` | :x: | :x: | _ | _ | EK, find data
| `EKGC_C` | :x: | :x: | _ | _ | EK, get event data, character
| `EKGD_C` | :x: | :x: | _ | _ | EK, get event data, double precision
| `EKGI_C` | :x: | :x: | _ | _ | EK, get event data, integer
| `EKIFLD_C` | :x: | :x: | _ | _ | EK, initialize segment for fast write
| `EKINSR_C` | :x: | :x: | _ | _ | EK, insert record into segment
| `EKLEF_C` | :x: | :x: | _ | _ | EK, load event file
| `EKNELT_C` | :x: | :x: | _ | _ | EK, get number of elements in column entry
| `EKNSEG_C` | :x: | :x: | _ | _ | EK, number of segments in file
| `EKNTAB_C` | :x: | :x: | _ | _ | EK, return number of loaded tables
| `EKOPN_C` | :x: | :x: | _ | _ | EK, open new file
| `EKOPR_C` | :x: | :x: | _ | _ | EK, open file for reading
| `EKOPS_C` | :x: | :x: | _ | _ | EK, open scratch file
| `EKOPW_C` | :x: | :x: | _ | _ | EK, open file for writing
| `EKPSEL_C` | :x: | :x: | _ | _ | EK, parse SELECT clause
| `EKRCEC_C` | :x: | :x: | _ | _ | EK, read column entry element, character
| `EKRCED_C` | :x: | :x: | _ | _ | EK, read column entry element, d.p.
| `EKRCEI_C` | :x: | :x: | _ | _ | EK, read column entry element, integer
| `EKSSUM_C` | :x: | :x: | _ | _ | EK, return segment summary
| `EKTNAM_C` | :x: | :x: | _ | _ | EK, return name of loaded table
| `EKUCEC_C` | :x: | :x: | _ | _ | EK, update character column entry
| `EKUCED_C` | :x: | :x: | _ | _ | EK, update d.p. column entry
| `EKUCEI_C` | :x: | :x: | _ | _ | EK, update integer column entry
| `EKUEF_C` | :x: | :x: | _ | _ | EK, unload event file
| `EL2CGV_C` | :x: | :x: | _ | _ | Ellipse to center and generating vectors
| `ELEMC_C` | :x: | :x: | _ | _ | Element of a character set
| `ELEMD_C` | :x: | :x: | _ | _ | Element of a double precision set
| `ELEMI_C` | :x: | :x: | _ | _ | Element of an integer set
| `EQNCPV_C` | :x: | :x: | _ | _ | Equinoctial Elements to position and velocity
| `EQSTR_C` | :x: | :x: | _ | _ | Equivalent strings
| `ERRACT_C` | :heavy_minus_sign: | :x: | _ | Used for rust-style error-handling. Thus not exposed. Internally its handled int `Spice::setup_error_handling` | Get/Set Default Error Action
| `ERRCH_C` | :heavy_minus_sign: | :x: | _ | Won't be implemented for now, as rust has its own error handling. | Insert String into Error Message Text
| `ERRDEV_C` | :heavy_minus_sign: | :x: | _ | Does not make much sense with internal error handling. Automatic printing of messages can be turned on and off with `Spice::disable_error_texts` and `Spice::enable_error_texts` however. | Get/Set Error Output Device Name
| `ERRDP_C` | :heavy_minus_sign: | :x: | _ | Won't be implemented for now, as rust has its own error handling. | Insert D.P. Number into Error Message Text
| `ERRINT_C` | :heavy_minus_sign: | :x: | _ | Won't be implemented for now, as rust has its own error handling. | Insert Integer into Error Message Text
| `ERRPRT_C` | :heavy_minus_sign: | :x: | _ | Won't be implemented for now, as rust has its own error handling. | Get/Set Error Output Items
| `ESRCHC_C` | :x: | :x: | _ | _ | Equivalence search, character
| `ET2LST_C` | :x: | :x: | _ | _ | ET to Local Solar Time
| `ET2UTC_C` | :heavy_check_mark: | :x: | `Spice::et2utc` | _ | Ephemeris Time to UTC
| `ETCAL_C` | :x: | :x: | _ | _ | Convert ET to Calendar format
| `EUL2M_C` | :x: | :heavy_check_mark: | _ | _ | Euler angles to matrix
| `EUL2XF_C` | :x: | :x: | _ | _ | Euler angles and derivative to transformation
| `EVSGP4_C` | :x: | :heavy_check_mark: | _ | _ | Evaluate "two-line" element data
| `EXISTS_C` | :x: | :x: | _ | _ | Does the file exist?
| `EXPOOL_C` | :x: | :x: | _ | _ | Confirm the existence of a pooled kernel variable

### F

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `FAILED_C` | :heavy_minus_sign: | :x: | _ | Used for rust-style error-handling. Thus not exposed. Internally its handled int `Spice::check_for_error` | Error Status Indicator
| `FILLD_C` | :x: | :x: | _ | _ | Fill a double precision array
| `FILLI_C` | :x: | :x: | _ | _ | Fill an integer array
| `FOVRAY_C` | :x: | :heavy_check_mark: | _ | _ | Is ray in FOV at time?
| `FOVTRG_C` | :x: | :heavy_check_mark: | _ | _ | Is target in FOV at time?
| `FRAME_C` | :x: | :x: | _ | _ | Build a right handed coordinate frame
| `FRINFO_C` | :x: | :x: | _ | _ | Frame Information
| `FRMNAM_C` | :x: | :x: | _ | _ | Frame to Name
| `FTNCLS_C` | :x: | :x: | _ | _ | Close file designated by Fortran unit
| `FURNSH_C` | :heavy_check_mark: | :heavy_check_mark: | `Spice::furnsh` | _ | Furnish a program with SPICE kernels

### F

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `GCPOOL_C` | :x: | :heavy_check_mark: | _ | _ | Get character data from the kernel pool
| `GDPOOL_C` | :x: | :heavy_check_mark: | _ | _ | Get d.p. values from the kernel pool
| `GEOREC_C` | :x: | :heavy_check_mark: | _ | _ | Geodetic to rectangular coordinates
| `GETCML_C` | :x: | :x: | _ | _ | Get the command line
| `GETELM_C` | :x: | :heavy_check_mark: | _ | _ | Get the components from two-line elements
| `GETFAT_C` | :x: | :x: | _ | _ | Get file architecture and type
| `GETFOV_C` | :x: | :heavy_check_mark: | _ | _ | Get instrument FOV parameters
| `GETFVN_C` | :x: | :heavy_check_mark: | _ | _ | Get instrument FOV parameters, by instrument name
| `GETMSG_C` | :heavy_minus_sign: | :x: | _ | Used for rust-style error-handling. Thus not exposed. Internally its handled int `Spice::check_for_error` | Get Error Message
| `GFBAIL_C` | :x: | :x: | _ | _ | GF, interrupt signal indicator
| `GFCLRH_C` | :x: | :x: | _ | _ | GF, clear interrupt signal handler status
| `GFDIST_C` | :x: | :heavy_check_mark: | _ | _ | GF, distance search
| `GFEVNT_C` | :x: | :x: | _ | _ | GF, geometric event finder
| `GFFOVE_C` | :x: | :x: | _ | _ | GF, is target in FOV?
| `GFILUM_C` | :x: | :heavy_check_mark: | _ | _ | GF, illumination angle search
| `GFINTH_C` | :x: | :x: | _ | _ | GF, interrupt signal handler
| `GFOCCE_C` | :x: | :x: | _ | _ | GF, occultation event
| `GFOCLT_C` | :x: | :heavy_check_mark: | _ | _ | GF, find occultation
| `GFPA_C` | :x: | :heavy_check_mark: | _ | _ | GF, phase angle search
| `GFPOSC_C` | :x: | :heavy_check_mark: | _ | _ | GF, observer-target vector coordinate search
| `GFREFN_C` | :x: | :x: | _ | _ | GF, default refinement estimator
| `GFREPF_C` | :x: | :x: | _ | _ | GF, progress report finalization
| `GFREPI_C` | :x: | :x: | _ | _ | GF, progress report initialization
| `GFREPU_C` | :x: | :x: | _ | _ | GF, progress report update
| `GFRFOV_C` | :x: | :heavy_check_mark: | _ | _ | GF, is ray in FOV?
| `GFRR_C` | :x: | :heavy_check_mark: | _ | _ | GF, range rate search
| `GFSEP_C` | :x: | :heavy_check_mark: | _ | _ | GF, angular separation search
| `GFSNTC_C` | :x: | :heavy_check_mark: | _ | _ | GF, surface intercept vector coordinate search
| `GFSSTP_C` | :x: | :x: | _ | _ | Geometry finder set step size
| `GFSTEP_C` | :x: | :x: | _ | _ | Geometry finder step size
| `GFSTOL_C` | :x: | :x: | _ | _ | GF, set a tolerance value for GF
| `GFSUBC_C` | :x: | :heavy_check_mark: | _ | _ | GF, subpoint vector coordinate search
| `GFTFOV_C` | :x: | :heavy_check_mark: | _ | _ | GF, is target in FOV?
| `GFUDB_C` | :x: | :x: | _ | _ | GF, user defined boolean
| `GFUDS_C` | :x: | :x: | _ | _ | GF, user defined scalar
| `GIPOOL_C` | :x: | :heavy_check_mark: | _ | _ | Get integers from the kernel pool
| `GNPOOL_C` | :x: | :x: | _ | _ | Get names of kernel pool variables

### H

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `HALFPI_C` | :x: | :heavy_check_mark: | _ | _ | Half the value of pi
| `HRMESP_C` | :x: | :x: | _ | _ | Hermite polynomial interpolation, equal spacing
| `HRMINT_C` | :x: | :x: | _ | _ | Hermite polynomial interpolation
| `HX2DP_C` | :x: | :x: | _ | _ | Hexadecimal string to d.p. number

### I

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `IDENT_C` | :x: | :x: | _ | _ | Return the 3x3 identity matrix
| `ILLUM_C` | :x: | :x: | _ | _ | Illumination angles
| `ILLUM_PL02` | :x: | :x: | _ | _ | illumination angles using DSK type 2 plate model
| `ILLUM_PLID_PL02` | :x: | :x: | _ | _ | illumination angles using type 2 DSK
| `ILLUMF_C` | :x: | :heavy_check_mark: | _ | _ | Illumination angles, general source, return flags
| `ILLUMG_C` | :x: | :heavy_check_mark: | _ | _ | Illumination angles, general source
| `ILUMIN_C` | :x: | :heavy_check_mark: | _ | _ | Illumination angles
| `INEDPL_C` | :x: | :x: | _ | _ | Intersection of ellipsoid and plane
| `INELPL_C` | :x: | :x: | _ | _ | Intersection of ellipse and plane
| `INRYPL_C` | :x: | :x: | _ | _ | Intersection of ray and plane
| `INSRTC_C` | :x: | :x: | _ | _ | Insert an item into a character set
| `INSRTD_C` | :x: | :x: | _ | _ | Insert an item into a double precision set
| `INSRTI_C` | :x: | :x: | _ | _ | Insert an item into an integer set
| `INTER_C` | :x: | :x: | _ | _ | Intersection of two sets
| `INTMAX_C` | :x: | :x: | _ | _ | Largest integer number
| `INTMIN_C` | :x: | :x: | _ | _ | Smallest integer number
| `INVERT_C` | :x: | :x: | _ | _ | Invert a 3x3 matrix
| `INVORT_C` | :x: | :x: | _ | _ | Invert nearly orthogonal matrices
| `INVSTM_C` | :x: | :x: | _ | _ | Inverse of state transformation matrix
| `ISORDV_C` | :x: | :x: | _ | _ | Is array an order vector?
| `ISRCHC_C` | :x: | :x: | _ | _ | Search in a character array
| `ISRCHD_C` | :x: | :x: | _ | _ | Search in a double precision array
| `ISRCHI_C` | :x: | :x: | _ | _ | Search in an integer array
| `ISROT_C` | :x: | :x: | _ | _ | Indicate whether a matrix is a rotation matrix
| `ISWHSP_C` | :x: | :x: | _ | _ | Determine whether a string is white space

### J

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `J1900_C` | :x: | :heavy_check_mark: | _ | _ | Julian Date of 1900.0 JAN 0.5
| `J1950_C` | :x: | :heavy_check_mark: | _ | _ | Julian Date of 1950.0 JAN 1.0
| `J2000_C` | :x: | :heavy_check_mark: | _ | _ | Julian Date of 2000 JAN 1.5
| `J2100_C` | :x: | :heavy_check_mark: | _ | _ | Julian Date of 2100 JAN 1.5
| `JYEAR_C` | :x: | :heavy_check_mark: | _ | _ | Seconds per julian year


### K

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `KCLEAR_C` | :x: | :x: | _ | _ | Keeper clear
| `KDATA_C` | :x: | :x: | _ | _ | Kernel Data
| `KINFO_C` | :x: | :x: | _ | _ | Kernel Information
| `KPLFRM_C` | :x: | :x: | _ | _ | Kernel pool frame IDs
| `KTOTAL_C` | :x: | :x: | _ | _ | Kernel Totals
| `KXTRCT_C` | :x: | :x: | _ | _ | Extract a substring starting with a keyword


### L

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `LASTNB_C` | :x: | :x: | _ | _ | Last non-blank character
| `LATCYL_C` | :x: | :heavy_check_mark: | _ | _ | Latitudinal to cylindrical coordinates
| `LATREC_C` | :x: | :heavy_check_mark: | _ | _ | Latitudinal to rectangular coordinates
| `LATSPH_C` | :x: | :heavy_check_mark: | _ | _ | Latitudinal to spherical coordinates
| `LATSRF_C` | :x: | :heavy_check_mark: | _ | _ | Latitudinal grid to surface points
| `LCASE_C` | :x: | :x: | _ | _ | Convert to lowercase
| `LDPOOL_C` | :x: | :x: | _ | _ | Load variables from a kernel file into the pool
| `LGRESP_C` | :x: | :x: | _ | _ | Lagrange interpolation on equally spaced points
| `LGRIND_C` | :x: | :x: | _ | _ | Lagrange polynomial interpolation with derivative
| `LGRINT_C` | :x: | :x: | _ | _ | Lagrange polynomial interpolation
| `LIMB_PL02` | :x: | :x: | _ | _ | Limb using DSK type 2 plate model
| `LIMBPT_C` | :x: | :heavy_check_mark: | _ | _ | Limb points on an extended object
| `LLGRID_PL02` | :x: | :x: | _ | _ | Lon/lat grid using DSK type 2 plate model
| `LMPOOL_C` | :x: | :x: | _ | _ | Load variables from memory into the pool
| `LPARSE_C` | :x: | :x: | _ | _ | Parse items from a list
| `LPARSM_C` | :x: | :x: | _ | _ | Parse a list of items having multiple delimiters
| `LPARSS_C` | :x: | :x: | _ | _ | Parse a list of items; return a set
| `LSPCN_C` | :x: | :x: | _ | _ | Longitude of the sun, planetocentric
| `LSTLEC_C` | :x: | :x: | _ | _ | Last character element less than or equal to.
| `LSTLED_C` | :x: | :x: | _ | _ | Last double precision element less than or equal
| `LSTLEI_C` | :x: | :x: | _ | _ | Last integer element less than or equal to
| `LSTLTC_C` | :x: | :x: | _ | _ | Last character element less than
| `LSTLTD_C` | :x: | :x: | _ | _ | Last double precision element less than
| `LSTLTI_C` | :x: | :x: | _ | _ | Last integer element less than
| `LTIME_C` | :x: | :x: | _ | _ | Light Time
| `LX4DEC_C` | :x: | :x: | _ | _ | Scan for decimal number
| `LX4NUM_C` | :x: | :x: | _ | _ | Scan for number
| `LX4SGN_C` | :x: | :x: | _ | _ | Scan for signed integer
| `LX4UNS_C` | :x: | :x: | _ | _ | Scan for unsigned integer
| `LXQSTR_C` | :x: | :x: | _ | _ | Lex quoted string

### M

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `M2EUL_C` | :x: | :heavy_check_mark: | _ | _ | Matrix to Euler angles
| `M2Q_C` | :x: | :heavy_check_mark: | _ | _ | Matrix to quaternion
| `MATCHI_C` | :x: | :x: | _ | _ | Match string against wildcard template
| `MATCHW_C` | :x: | :x: | _ | _ | Match string against wildcard template
| `MAXD_C` | :x: | :x: | _ | _ | Maximum of a set of double precision values
| `MAXI_C` | :x: | :x: | _ | _ | Maximum of a set of integers
| `MEQU_C` | :x: | :heavy_check_mark: | _ | _ | Matrix equal to another, 3x3
| `MEQUG_C` | :x: | :x: | _ | _ | Matrix equal to another, general dimension
| `MIND_C` | :x: | :x: | _ | _ | Minimum of a set of double precision values
| `MINI_C` | :x: | :x: | _ | _ | minimum of a set of integers
| `MOVED_C` | :x: | :x: | _ | _ | Move a double precision array to another
| `MTXM_C` | :x: | :heavy_check_mark: | _ | _ | Matrix transpose times matrix, 3x3
| `MTXMG_C` | :x: | :x: | _ | _ | Matrix transpose times matrix, general dimension
| `MTXV_C` | :x: | :heavy_check_mark: | _ | _ | Matrix transpose times vector, 3x3
| `MTXVG_C` | :x: | :x: | _ | _ | Matrix transpose times vector, general dimension
| `MXM_C` | :x: | :heavy_check_mark: | _ | _ | Matrix times matrix, 3x3
| `MXMG_C` | :x: | :x: | _ | _ | Matrix times matrix, general dimension
| `MXMT_C` | :x: | :heavy_check_mark: | _ | _ | Matrix times matrix transpose, 3x3
| `MXMTG_C` | :x: | :x: | _ | _ | Matrix times matrix transpose, general dimension
| `MXV_C` | :x: | :heavy_check_mark: | _ | _ | Matrix times vector, 3x3
| `MXVG_C` | :x: | :x: | _ | _ | Matrix times vector, general dimension

### N

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `NAMFRM_C` | :x: | :x: | _ | _ | Name to frame
| `NCPOS_C` | :x: | :x: | _ | _ | NOT Character position
| `NCPOSR_C` | :x: | :x: | _ | _ | Character position, reverse
| `NEARPT_C` | :x: | :x: | _ | _ | Nearest point on an ellipsoid
| `NEXTWD_C` | :x: | :x: | _ | _ | Next word in a character string
| `NPEDLN_C` | :x: | :x: | _ | _ | Nearest point on ellipsoid to line
| `NPELPT_C` | :x: | :x: | _ | _ | Nearest point on ellipse to point
| `NPLNPT_C` | :x: | :x: | _ | _ | Nearest point on line to point
| `NTHWD_C` | :x: | :x: | _ | _ | n'th word in a character string
| `NVC2PL_C` | :x: | :x: | _ | _ | Normal vector and constant to plane
| `NVP2PL_C` | :x: | :x: | _ | _ | Normal vector and point to plane

### O

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `OCCULT_C` | :x: | :heavy_check_mark: | _ | _ | find occultation type at time
| `ORDC_C` | :x: | :x: | _ | _ | The ordinal position of an element in a set
| `ORDD_C` | :x: | :x: | _ | _ | The ordinal position of an element in a set
| `ORDERC_C` | :x: | :x: | _ | _ | Order of a character array
| `ORDERD_C` | :x: | :x: | _ | _ | Order of a double precision array
| `ORDERI_C` | :x: | :x: | _ | _ | Order of an integer array
| `ORDI_C` | :x: | :x: | _ | _ | The ordinal position of an element in a set
| `OSCELT_C` | :x: | :heavy_check_mark: | _ | _ | Determine conic elements from state
| `OSCLTX_C` | :x: | :x: | _ | _ | Extended osculating elements from state

### P

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `PCKCLS_C` | :x: | :x: | _ | _ | PCK, close file
| `PCKCOV_C` | :x: | :heavy_check_mark: | _ | _ | PCK coverage
| `PCKFRM_C` | :x: | :heavy_check_mark: | _ | _ | PCK, get reference frame class ID set
| `PCKLOF_C` | :x: | :x: | _ | _ | PCK, load binary file
| `PCKOPN_C` | :x: | :x: | _ | _ | PCK, open new file
| `PCKUOF_C` | :x: | :x: | _ | _ | PCK, unload binary file
| `PCKW02_C` | :x: | :x: | _ | _ | PCK, write type 2 segment
| `PCPOOL_C` | :x: | :x: | _ | _ | Put character strings into the kernel pool
| `PDPOOL_C` | :x: | :x: | _ | _ | Put d.p.'s into the kernel pool
| `PGRREC_C` | :x: | :heavy_check_mark: | _ | _ | Planetographic to rectangular
| `PHASEQ_C` | :x: | :heavy_check_mark: | _ | _ | Phase angle quantity between bodies centers
| `PI_C` | :x: | :heavy_check_mark: | _ | _ | Value of pi
| `PIPOOL_C` | :x: | :x: | _ | _ | Put integers into the kernel pool
| `PJELPL_C` | :x: | :x: | _ | _ | Project ellipse onto plane
| `PL2NVC_C` | :x: | :x: | _ | _ | Plane to normal vector and constant
| `PL2NVP_C` | :x: | :x: | _ | _ | Plane to normal vector and point
| `PL2PSV_C` | :x: | :x: | _ | _ | Plane to point and spanning vectors
| `PLTAR_C` | :x: | :x: | _ | _ | Compute area of plate set
| `PLTEXP_C` | :x: | :x: | _ | _ | Plate expander
| `PLTNP_C` | :x: | :x: | _ | _ | Nearest point on triangular plate
| `PLTNRM_C` | :x: | :x: | _ | _ | DSK, compute outward normal of plate
| `PLTVOL_C` | :x: | :x: | _ | _ | Compute volume of plate model
| `POLYDS_C` | :x: | :x: | _ | _ | Compute a Polynomial and its Derivatives
| `POS_C` | :x: | :x: | _ | _ | Position of substring
| `POSR_C` | :x: | :x: | _ | _ | Position of substring, reverse search
| `PROMPT_C` | :x: | :x: | _ | _ | Prompt a user for a string
| `PROP2B_C` | :x: | :x: | _ | _ | Propagate a two-body solution
| `PRSDP_C` | :x: | :x: | _ | _ | Parse d.p. number with error checking
| `PRSINT_C` | :x: | :x: | _ | _ | Parse integer with error checking
| `PSV2PL_C` | :x: | :x: | _ | _ | Point and spanning vectors to plane
| `PUTCML_C` | :x: | :x: | _ | _ | Get the command line
| `PXFORM_C` | :x: | :heavy_check_mark: | _ | _ | Position Transformation Matrix
| `PXFRM2_C` | :x: | :heavy_check_mark: | _ | _ | Position Transform Matrix, Different Epochs

### Q

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `Q2M_C` | :x: | :heavy_check_mark: | _ | _ | Quaternion to matrix
| `QCKTRC_C` | :heavy_minus_sign: | :x: | _ | Used for rust-style error-handling. Thus not exposed. Internally its handled int `Spice::check_for_error` | Get Quick Traceback
| `QDERIV_C` | :x: | :x: | _ | _ | Quadratic derivative
| `QDQ2AV_C` | :x: | :x: | _ | _ | Quaternion and quaternion derivative to a.v.
| `QXQ_C` | :x: | :x: | _ | _ | Quaternion times quaternion

### R

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `RADREC_C` | :x: | :heavy_check_mark: | _ | _ | Range, RA and DEC to rectangular coordinates
| `RAV2XF_C` | :x: | :x: | _ | _ | Rotation and angular velocity to transform
| `RAXISA_C` | :x: | :heavy_check_mark: | _ | _ | Rotation axis of a matrix
| `RDTEXT_C` | :x: | :x: | _ | _ | Read a line from a text file
| `RECAZL_C` | :x: | :heavy_check_mark: | _ | _ | Rectangular coordinates to AZ/EL
| `RECCYL_C` | :x: | :heavy_check_mark: | _ | _ | Rectangular to cylindrical coordinates
| `RECGEO_C` | :x: | :heavy_check_mark: | _ | _ | Rectangular to geodetic
| `RECLAT_C` | :x: | :heavy_check_mark: | _ | _ | Rectangular to latitudinal coordinates
| `RECPGR_C` | :x: | :heavy_check_mark: | _ | _ | Rectangular to planetographic
| `RECRAD_C` | :x: | :heavy_check_mark: | _ | _ | Rectangular coordinates to RA and DEC
| `RECSPH_C` | :x: | :heavy_check_mark: | _ | _ | Rectangular to spherical coordinates
| `REMOVC_C` | :x: | :x: | _ | _ | Remove an item from a character set
| `REMOVD_C` | :x: | :x: | _ | _ | Remove an item from a double precision set
| `REMOVI_C` | :x: | :x: | _ | _ | Remove an item from an integer set
| `REORDC_C` | :x: | :x: | _ | _ | Reorder a character array
| `REORDD_C` | :x: | :x: | _ | _ | Reorder a double precision array
| `REORDI_C` | :x: | :x: | _ | _ | Reorder an integer array
| `REORDL_C` | :x: | :x: | _ | _ | Reorder a logical array
| `REPMC_C` | :x: | :x: | _ | _ | Replace marker with character string
| `REPMCT_C` | :x: | :x: | _ | _ | Replace marker with cardinal text
| `REPMD_C` | :x: | :x: | _ | _ | Replace marker with double precision number
| `REPMF_C` | :x: | :x: | _ | _ | Replace marker with formatted d.p. value
| `REPMI_C` | :x: | :x: | _ | _ | Replace marker with integer
| `REPML_C` | :x: | :x: | _ | _ | Replace marker with logical value text
| `REPMOT_C` | :x: | :x: | _ | _ | Replace marker with ordinal text
| `RESET_C` | :heay_minus_sign: | :x: | _ | Used for rust-style error-handling. Thus not exposed. Internally its handled in `Spice::check_for_error` | Reset Error Status
| `RETURN_C` | :x: | :x: | _ | _ | Immediate Return Indicator
| `ROTATE_C` | :x: | :heavy_check_mark: | _ | _ | Generate a rotation matrix
| `ROTMAT_C` | :x: | :heavy_check_mark: | _ | _ | Rotate a matrix
| `ROTVEC_C` | :x: | :heavy_check_mark: | _ | _ | Transform a vector via a rotation
| `RPD_C` | :x: | :heavy_check_mark: | _ | _ | Radians per degree
| `RQUAD_C` | :x: | :x: | _ | _ | Roots of a quadratic equation

### S

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `SAELGV_C` | :x: | :x: | _ | _ | Semi-axes of ellipse from generating vectors
| `SCARD_C` | :heavy_minus_sign: | :x: | _ | Not needed because cells are re-implemented using vectors | Set the cardinality of a cell
| `SCDECD_C` | :x: | :heavy_check_mark: | _ | _ | Decode spacecraft clock
| `SCE2C_C` | :x: | :x: | _ | _ | ET to continuous SCLK ticks
| `SCE2S_C` | :x: | :heavy_check_mark: | _ | _ | ET to SCLK string
| `SCE2T_C` | :x: | :x: | _ | _ | ET to SCLK ticks
| `SCENCD_C` | :x: | :heavy_check_mark: | _ | _ | Encode spacecraft clock
| `SCFMT_C` | :x: | :x: | _ | _ | Convert SCLK "ticks" to character clock format
| `SCPART_C` | :x: | :x: | _ | _ | Spacecraft Clock Partition Information
| `SCS2E_C` | :x: | :heavy_check_mark: | _ | _ | SCLK string to ET
| `SCT2E_C` | :x: | :heavy_check_mark: | _ | _ | SCLK ticks to ET
| `SCTIKS_C` | :x: | :x: | _ | _ | Convert spacecraft clock string to ticks.
| `SDIFF_C` | :x: | :x: | _ | _ | Symmetric difference of two sets
| `SET_C` | :x: | :x: | _ | _ | Compare sets
| `SETMSG_C` | :heavy_minus_sign: | :x: | _ | Won't be implemented for now, as rust has its own error handling. | Set Long Error Message
| `SHELLC_C` | :x: | :x: | _ | _ | Shell sort a character array
| `SHELLD_C` | :x: | :x: | _ | _ | Shell sort a double precision array
| `SHELLI_C` | :x: | :x: | _ | _ | Shell sort an integer array
| `SIGERR_C` | :heavy_minus_sign: | :x: | _ | Won't be implemented for now, as rust has its own error handling. | Signal Error Condition
| `SIZE_C` | :heavy_minus_sign: | :x: | _ | Not required as cells are re-implemented with vectors | Size of a cell
| `SPD_C` | :x: | :heavy_check_mark: | _ | _ | Seconds per day
| `SPHCYL_C` | :x: | :heavy_check_mark: | _ | _ | Spherical to cylindrical coordinates
| `SPHLAT_C` | :x: | :heavy_check_mark: | _ | _ | Spherical to latitudinal coordinates
| `SPHREC_C` | :x: | :heavy_check_mark: | _ | _ | Spherical to rectangular coordinates
| `SPK14A_C` | :x: | :x: | _ | _ | SPK, add data to a type 14 segment
| `SPK14B_C` | :x: | :x: | _ | _ | SPK, begin a type 14 segment
| `SPK14E_C` | :x: | :x: | _ | _ | SPK, end a type 14 segment
| `SPKACS_C` | :x: | :x: | _ | _ | SPK, aberration corrected state
| `SPKAPO_C` | :x: | :x: | _ | _ | S/P Kernel, apparent position only
| `SPKAPP_C` | :x: | :x: | _ | _ | S/P Kernel, apparent state
| `SPKAPS_C` | :x: | :x: | _ | _ | SPK, apparent state
| `SPKCLS_C` | :x: | :x: | _ | _ | SPK, Close file
| `SPKCOV_C` | :x: | :heavy_check_mark: | _ | _ | SPK coverage
| `SPKCPO_C` | :x: | :heavy_check_mark: | _ | _ | SPK, constant position observer state
| `SPKCPT_C` | :x: | :heavy_check_mark: | _ | _ | SPK, constant position target state
| `SPKCVO_C` | :x: | :heavy_check_mark: | _ | _ | SPK, constant velocity observer state
| `SPKCVT_C` | :x: | :heavy_check_mark: | _ | _ | SPK, constant velocity target state
| `SPKEZ_C` | :x: | :x: | _ | _ | S/P Kernel, easy reader
| `SPKEZP_C` | :x: | :x: | _ | _ | S/P Kernel, easy position
| `SPKEZR_C` | :x: | :heavy_check_mark: | _ | _ | S/P Kernel, easier reader
| `SPKGEO_C` | :x: | :x: | _ | _ | S/P Kernel, geometric state
| `SPKGPS_C` | :x: | :x: | _ | _ | S/P Kernel, geometric position
| `SPKLEF_C` | :x: | :x: | _ | _ | S/P Kernel, Load ephemeris file
| `SPKLTC_C` | :x: | :x: | _ | _ | S/P Kernel, light time corrected state
| `SPKOBJ_C` | :x: | :heavy_check_mark: | _ | _ | SPK objects
| `SPKOPA_C` | :x: | :x: | _ | _ | SPK open for addition
| `SPKOPN_C` | :x: | :x: | _ | _ | SPK, open new file.
| `SPKPDS_C` | :x: | :x: | _ | _ | SPK pack descriptor
| `SPKPOS_C` | :x: | :heavy_check_mark: | _ | _ | S/P Kernel, position
| `SPKPVN_C` | :x: | :x: | _ | _ | S/P Kernel, position and velocity in native frame
| `SPKSFS_C` | :x: | :x: | _ | _ | S/P Kernel, Select file and segment
| `SPKSSB_C` | :x: | :x: | _ | _ | S/P Kernel, solar system barycenter
| `SPKSUB_C` | :x: | :x: | _ | _ | S/P Kernel, subset
| `SPKUDS_C` | :x: | :x: | _ | _ | SPK | :x: | :x: | _ | _ | unpack segment descriptor
| `SPKUEF_C` | :x: | :x: | _ | _ | SPK Kernel, Unload ephemeris file
| `SPKW02_C` | :x: | :x: | _ | _ | Write SPK segment, type 2
| `SPKW03_C` | :x: | :x: | _ | _ | Write SPK segment, type 3
| `SPKW05_C` | :x: | :x: | _ | _ | Write SPK segment, type 5
| `SPKW08_C` | :x: | :x: | _ | _ | Write SPK segment, type 8
| `SPKW09_C` | :x: | :x: | _ | _ | Write SPK segment, type 9
| `SPKW10_C` | :x: | :x: | _ | _ | SPK | :x: | :x: | _ | _ | write a type 10 segment
| `SPKW12_C` | :x: | :x: | _ | _ | Write SPK segment, type 12
| `SPKW13_C` | :x: | :x: | _ | _ | Write SPK segment, type 13
| `SPKW15_C` | :x: | :x: | _ | _ | SPK, write a type 15 segment
| `SPKW17_C` | :x: | :x: | _ | _ | SPK, write a type 17 segment
| `SPKW18_C` | :x: | :x: | _ | _ | Write SPK segment, type 18
| `SPKW20_C` | :x: | :x: | _ | _ | Write SPK segment, type 20
| `SRFC2S_C` | :x: | :heavy_check_mark: | _ | _ | Surface and body ID codes to surface string
| `SRFCSS_C` | :x: | :heavy_check_mark: | _ | _ | Surface ID and body string to surface string
| `SRFNRM_C` | :x: | :heavy_check_mark: | _ | _ | Map surface points to outward normal vectors
| `SRFREC_C` | :x: | :heavy_check_mark: | _ | _ | Surface to rectangular coordinates
| `SRFS2C_C` | :x: | :heavy_check_mark: | _ | _ | Surface and body strings to surface ID code
| `SRFSCC_C` | :x: | :heavy_check_mark: | _ | _ | Surface string and body ID code to surface ID code
| `SRFXPT_C` | :x: | :x: | _ | _ | Surface intercept point
| `SSIZE_C` | :heavy_minus_sign: | :x: | _ | Not required as cells are re-implemented using vectors | Set the size of a cell
| `STELAB_C` | :x: | :x: | _ | _ | Stellar Aberration
| `STLABX_C` | :x: | :x: | _ | _ | Stellar aberration, transmission case
| `STPOOL_C` | :x: | :x: | _ | _ | String from pool
| `STR2ET_C` | :heavy_check_mark: | :heavy_check_mark: | `Spice::str2et` | _ | String to ET
| `SUBPNT_C` | :x: | :heavy_check_mark: | _ | _ | Sub-observer point
| `SUBPT_C` | :x: | :x: | _ | _ | Sub-observer point
| `SUBPT_PL02` | :x: | :x: | _ | _ | Sub-observer point using DSK type 2 plate model
| `SUBSLR_C` | :x: | :heavy_check_mark: | _ | _ | Sub-solar point
| `SUBSOL_C` | :x: | :x: | _ | _ | Sub-solar point
| `SUBSOL_PL02` | :x: | :x: | _ | _ | Sub-solar point using DSK type 2 plate model
| `SUMAD_C` | :x: | :x: | _ | _ | Sum of a double precision array
| `SUMAI_C` | :x: | :x: | _ | _ | Sum of an integer array
| `SURFNM_C` | :x: | :x: | _ | _ | Surface normal vector on an ellipsoid
| `SURFPT_C` | :x: | :x: | _ | _ | Surface point on an ellipsoid
| `SURFPV_C` | :x: | :x: | _ | _ | Surface point and velocity
| `SWPOOL_C` | :x: | :x: | _ | _ | Set watch on a pool variable
| `SXFORM_C` | :x: | :heavy_check_mark: | _ | _ | State Transformation Matrix
| `SZPOOL_C` | :x: | :x: | _ | _ | Get size limitations of the kernel pool

### T

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `TANGPT_C` | :x: | :heavy_check_mark: | _ | _ | Ray-ellipsoid tangent point
| `TERM_PL02` | :x: | :x: | _ | _ | Terminator using DSK type 2 plate model
| `TERMPT_C` | :x: | :heavy_check_mark: | _ | _ | Terminator points on an extended object
| `TIMDEF_C` | :x: | :x: | _ | _ | Time Software Defaults
| `TIMOUT_C` | :heavy_check_mark: | :heavy_check_mark: | `Spice::timout` | _ | Time Output
| `TIPBOD_C` | :x: | :x: | _ | _ | Transformation, inertial position to bodyfixed
| `TISBOD_C` | :x: | :x: | _ | _ | Transformation, inertial state to bodyfixed
| `TKFRAM_C` | :x: | :x: | _ | _ | TK frame, find position rotation
| `TKVRSN_C` | :x: | :x: | _ | _ | Toolkit version strings
| `TPARCH_C` | :x: | :x: | _ | _ | Parse check---check format of strings
| `TPARSE_C` | :x: | :x: | _ | _ | Parse a UTC time string
| `TPICTR_C` | :x: | :x: | _ | _ | Create a Time Format Picture
| `TRACE_C` | :x: | :heavy_check_mark: | _ | _ | Trace of a 3x3 matrix
| `TRCDEP_C` | :x: | :x: | _ | _ | Traceback depth
| `TRCNAM_C` | :x: | :x: | _ | _ | Get module name from traceback
| `TRCOFF_C` | :x: | :x: | _ | _ | Turn tracing off
| `TRGSEP_C` | :x: | :heavy_check_mark: | _ | _ | Separation quantity from observer
| `TSETYR_C` | :x: | :x: | _ | _ | Time --- set year expansion boundaries
| `TWOPI_C` | :x: | :heavy_check_mark: | _ | _ | Twice the value of pi
| `TWOVEC_C` | :x: | :heavy_check_mark: | _ | _ | Two vectors defining an orthonormal frame
| `TWOVXF_C` | :x: | :x: | _ | _ | Two states defining a frame transformation
| `TYEAR_C` | :x: | :heavy_check_mark: | _ | _ | Seconds per tropical year

### U

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `UCASE_C` | :x: | :x: | _ | _ | Convert to uppercase
| `UCRSS_C` | :x: | :x: | _ | _ | Unitized cross product, 3x3
| `UDDC_C` | :x: | :x: | _ | _ | Derivative of function less than zero, df(x)/dx < 0
| `UDDF_C` | :x: | :x: | _ | _ | First derivative of a function, df(x)/dx
| `UDF_C` | :x: | :x: | _ | _ | GF, dummy function
| `UNION_C` | :x: | :x: | _ | _ | Union of two sets
| `UNITIM_C` | :x: | :x: | _ | _ | Uniform time scale transformation
| `UNLOAD_C` | :heavy_check_mark: | :heavy_check_mark: | _ | `Spice::unload` | Unload a kernel
| `UNORM_C` | :x: | :heavy_check_mark: | _ | _ | Unit vector and norm, 3 dimensional
| `UNORMG_C` | :x: | :x: | _ | _ | Unit vector and norm, general dimension
| `UTC2ET_C` | :x: | :x: | _ | _ | UTC to Ephemeris Time

### V

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `VADD_C` | :x: | :heavy_check_mark: | _ | _ | Vector addition, 3 dimensional
| `VADDG_C` | :x: | :x: | _ | _ | Vector addition, general dimension
| `VALID_C` | :x: | :x: | _ | _ | Validate a set
| `VCRSS_C` | :x: | :heavy_check_mark: | _ | _ | Vector cross product, 3 dimensions
| `VDIST_C` | :x: | :heavy_check_mark: | _ | _ | Vector distance
| `VDISTG_C` | :x: | :x: | _ | _ | Vector distance, general dimension
| `VDOT_C` | :x: | :x: | _ | _ | Vector dot product, 3 dimensions
| `VDOTG_C` | :x: | :x: | _ | _ | Vector dot product, general dimension
| `VEQU_C` | :x: | :heavy_check_mark: | _ | _ | Vector equality, 3 dimensions
| `VEQUG_C` | :x: | :x: | _ | _ | Vector equality, general dimension
| `VHAT_C` | :x: | :x: | _ | _ | "V-Hat", unit vector along V, 3 dimensions
| `VHATG_C` | :x: | :x: | _ | _ | "V-Hat", unit vector along V, general dimension
| `VLCOM3_C` | :x: | :heavy_check_mark: | _ | _ | Vector linear combination, 3 dimensions
| `VLCOM_C` | :x: | :heavy_check_mark: | _ | _ | Vector linear combination, 3 dimensions
| `VLCOMG_C` | :x: | :x: | _ | _ | Vector linear combination, general dimension
| `VMINUG_C` | :x: | :x: | _ | _ | Negate vector, "-v", general dimension
| `VMINUS_C` | :x: | :heavy_check_mark: | _ | _ | Negate vector, "-v", 3 dimensions
| `VNORM_C` | :x: | :heavy_check_mark: | _ | _ | Vector norm, 3 dimensions
| `VNORMG_C` | :x: | :x: | _ | _ | Vector norm, general dimension
| `VPACK_C` | :x: | :heavy_check_mark: | _ | _ | Pack three scalar components into a vector
| `VPERP_C` | :x: | :heavy_check_mark: | _ | _ | Perpendicular component of a 3-vector
| `VPRJP_C` | :x: | :x: | _ | _ | Vector projection onto plane
| `VPRJPI_C` | :x: | :x: | _ | _ | Vector projection onto plane, inverted
| `VPROJ_C` | :x: | :heavy_check_mark: | _ | _ | Vector projection, 3 dimensions
| `VPROJG_C` | :x: | :x: | _ | _ | Vector projection, general dimension
| `VREL_C` | :x: | :heavy_check_mark: | _ | _ | Vector relative difference, 3 dimensions
| `VRELG_C` | :x: | :x: | _ | _ | Vector relative difference, general dimension
| `VROTV_C` | :x: | :heavy_check_mark: | _ | _ | Vector rotation about an axis
| `VSCL_C` | :x: | :heavy_check_mark: | _ | _ | Vector scaling, 3 dimensions
| `VSCLG_C` | :x: | :x: | _ | _ | Vector scaling, general dimension
| `VSEP_C` | :x: | :heavy_check_mark: | _ | _ | Angular separation of vectors, 3 dimensions
| `VSEPG_C` | :x: | :x: | _ | _ | Angular separation of vectors, general dimension
| `VSUB_C` | :x: | :heavy_check_mark: | _ | _ | Vector subtraction, 3 dimensions
| `VSUBG_C` | :x: | :x: | _ | _ | Vector subtraction, general dimension
| `VTMV_C` | :x: | :heavy_check_mark: | _ | _ | Vector transpose times matrix times vector, 3 dim
| `VTMVG_C` | :x: | :x: | _ | _ | Vector transpose times matrix times vector
| `VUPACK_C` | :x: | :heavy_check_mark: | _ | _ | Unpack three scalar components from a vector
| `VZERO_C` | :x: | :heavy_check_mark: | _ | _ | Is a vector the zero vector?
| `VZEROG_C` | :x: | :x: | _ | _ | Is a vector the zero vector? -- general dim.

### W

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `WNCARD_C` | :x: | :x: | _ | _ | Cardinality of a double precision window
| `WNCOMD_C` | :x: | :x: | _ | _ | Complement a DP window
| `WNCOND_C` | :x: | :x: | _ | _ | Contract the intervals of a DP window
| `WNDIFD_C` | :x: | :x: | _ | _ | Difference two DP windows
| `WNELMD_C` | :x: | :x: | _ | _ | Element of a DP window
| `WNEXPD_C` | :x: | :x: | _ | _ | Expand the intervals of a DP window
| `WNEXTD_C` | :x: | :x: | _ | _ | Extract the endpoints from a DP window
| `WNFETD_C` | :x: | :x: | _ | _ | Fetch an interval from a DP window
| `WNFILD_C` | :x: | :x: | _ | _ | Fill small gaps in a DP window
| `WNFLTD_C` | :x: | :x: | _ | _ | Filter small intervals from a DP window
| `WNINCD_C` | :x: | :x: | _ | _ | Included in a double precision window
| `WNINSD_C` | :x: | :x: | _ | _ | Insert an interval into a DP window
| `WNINTD_C` | :x: | :x: | _ | _ | Intersect two DP windows
| `WNRELD_C` | :x: | :x: | _ | _ | Compare two DP windows
| `WNSUMD_C` | :x: | :x: | _ | _ | Summary of a double precision window
| `WNUNID_C` | :x: | :x: | _ | _ | Union two DP windows
| `WNVALD_C` | :x: | :x: | _ | _ | Validate a DP window

### X

| SPICE API | Implemented | mostused | Rust API | Implementation Notes | Short Docstring |
|---|---|---|---|---|---|
| `XF2EUL_C` | :x: | :x: | _ | _ | State transformation to Euler angles
| `XF2RAV_C` | :x: | :x: | _ | _ | Transform to rotation and angular velocity
| `XFMSTA_C` | :x: | :heavy_check_mark: | _ | _ | Transform state between coordinate systems
| `XPOSE6_C` | :x: | :x: | _ | _ | Transpose a matrix, 6x6
| `XPOSE_C` | :x: | :heavy_check_mark: | _ | _ | Transpose a matrix, 3x3
| `XPOSEG_C` | :x: | :x: | _ | _ | Transpose a matrix, general

